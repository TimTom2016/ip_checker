use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct NetworkAddress(String);

#[derive(Debug, Error)]
pub enum NetworkAddressError {
    #[error("Invalid number of octets: expected 4, got {0}")]
    InvalidOctetCount(usize),

    #[error("Invalid octet at position {position}: {reason}")]
    InvalidOctet { position: usize, reason: String },

    #[error("Invalid network address: host bits must be 0 for prefix /{prefix}")]
    InvalidHostBits { prefix: u8 },

    #[error("Empty network address")]
    EmptyAddress,
}

impl PartialEq for NetworkAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl NetworkAddress {
    pub fn new(address: String) -> Self {
        Self::try_new(address, 24).unwrap() // Default /24 prefix
    }

    pub fn try_new(address: String, prefix: u8) -> Result<Self, NetworkAddressError> {
        if address.is_empty() {
            return Err(NetworkAddressError::EmptyAddress);
        }

        let octets: Vec<&str> = address.split('.').collect();
        if octets.len() != 4 {
            return Err(NetworkAddressError::InvalidOctetCount(octets.len()));
        }

        // Convert address to binary format for host bits checking
        let mut binary_addr: u32 = 0;
        for (i, octet) in octets.iter().enumerate() {
            match octet.parse::<u8>() {
                Ok(num) => {
                    binary_addr |= (num as u32) << (24 - (i * 8));
                }
                Err(_) => {
                    return Err(NetworkAddressError::InvalidOctet {
                        position: i + 1,
                        reason: format!("'{}' is not a valid number", octet),
                    });
                }
            }
        }

        // Check if host bits are all 0
        let host_bits_mask = (1u32 << (32 - prefix)) - 1;
        if (binary_addr & host_bits_mask) != 0 {
            return Err(NetworkAddressError::InvalidHostBits { prefix });
        }

        Ok(Self(address))
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }

    pub fn to_u32(&self) -> u32 {
        let octets: Vec<u8> = self
            .0
            .split('.')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        ((octets[0] as u32) << 24)
            | ((octets[1] as u32) << 16)
            | ((octets[2] as u32) << 8)
            | (octets[3] as u32)
    }

    pub fn from_u32(addr: u32, prefix: u8) -> Result<Self, NetworkAddressError> {
        // Ensure host bits are 0
        let host_bits_mask = (1u32 << (32 - prefix)) - 1;
        let network_addr = addr & !host_bits_mask;

        let address = format!(
            "{}.{}.{}.{}",
            (network_addr >> 24) & 0xFF,
            (network_addr >> 16) & 0xFF,
            (network_addr >> 8) & 0xFF,
            network_addr & 0xFF
        );

        Self::try_new(address, prefix)
    }
}

impl FromStr for NetworkAddress {
    type Err = NetworkAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new(s.to_string(), 24) // Default /24 prefix
    }
}

impl AsRef<str> for NetworkAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NetworkAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_network_address() {
        let addr = NetworkAddress::try_new("192.168.1.0".to_string(), 24);
        assert!(addr.is_ok());
    }

    #[test]
    fn test_invalid_host_bits() {
        let addr = NetworkAddress::try_new("192.168.1.1".to_string(), 24);
        assert!(matches!(
            addr.unwrap_err(),
            NetworkAddressError::InvalidHostBits { prefix: 24 }
        ));
    }

    #[test]
    fn test_invalid_octet_count() {
        let addr = NetworkAddress::try_new("192.168.1".to_string(), 24);
        assert!(matches!(
            addr.unwrap_err(),
            NetworkAddressError::InvalidOctetCount(3)
        ));
    }

    #[test]
    fn test_invalid_octet() {
        let addr = NetworkAddress::try_new("192.168.abc.0".to_string(), 24);
        assert!(matches!(
            addr.unwrap_err(),
            NetworkAddressError::InvalidOctet { position: 3, .. }
        ));
    }

    #[test]
    fn test_empty_address() {
        let addr = NetworkAddress::try_new("".to_string(), 24);
        assert!(matches!(
            addr.unwrap_err(),
            NetworkAddressError::EmptyAddress
        ));
    }

    #[test]
    fn test_from_u32() {
        let addr = NetworkAddress::from_u32(0xC0A80100, 24); // 192.168.1.0/24
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().as_string(), "192.168.1.0");
    }

    #[test]
    fn test_to_u32() {
        let addr = NetworkAddress::new("192.168.1.0".to_string());
        assert_eq!(addr.to_u32(), 0xC0A80100);
    }
}
