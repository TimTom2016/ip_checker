use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct IpAddress(String);

#[derive(Debug, Error)]
pub enum IpAddressError {
    #[error("Invalid number of octets: expected 4, got {0}")]
    InvalidOctetCount(usize),

    #[error("Invalid octet at position {position}: {reason}")]
    InvalidOctet { position: usize, reason: String },

    #[error("Octet out of range at position {position}: value must be between 1 and 255")]
    OctetOutOfRange { position: usize },

    #[error("Empty IP address")]
    EmptyAddress,
}

impl PartialEq for IpAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl IpAddress {
    pub fn new(ip: String) -> Self {
        Self::try_new(ip).unwrap()
    }

    pub fn try_new(ip: String) -> Result<Self, IpAddressError> {
        if ip.is_empty() {
            return Err(IpAddressError::EmptyAddress);
        }

        let octets: Vec<&str> = ip.split('.').collect();
        if octets.len() != 4 {
            return Err(IpAddressError::InvalidOctetCount(octets.len()));
        }

        for (i, octet) in octets.iter().enumerate() {
            // First try to parse as u16 to check if it's too large
            match octet.parse::<u16>() {
                Ok(num) if num > 255 => {
                    return Err(IpAddressError::OctetOutOfRange { position: i + 1 });
                }
                Ok(num) if num < 1 => {
                    return Err(IpAddressError::OctetOutOfRange { position: i + 1 });
                }
                Ok(_) => (), // Valid range, continue
                Err(_) => {
                    return Err(IpAddressError::InvalidOctet {
                        position: i + 1,
                        reason: format!("'{}' is not a valid number", octet),
                    });
                }
            }
        }

        Ok(Self(ip))
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

impl FromStr for IpAddress {
    type Err = IpAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new(s.to_string())
    }
}

impl AsRef<str> for IpAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for IpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid_ip() {
        let ip = IpAddress::try_new("192.168.1.1".to_string());
        assert!(ip.is_ok());
    }

    #[test]
    fn test_invalid_octet_count() {
        let ip = IpAddress::try_new("192.168.1".to_string());
        assert!(matches!(
            ip.unwrap_err(),
            IpAddressError::InvalidOctetCount(3)
        ));
    }

    #[test]
    fn test_invalid_octet() {
        let ip = IpAddress::try_new("192.168.abc.1".to_string());
        assert!(matches!(
            ip.unwrap_err(),
            IpAddressError::InvalidOctet { position: 3, .. }
        ));
    }

    #[test]
    fn test_octet_out_of_range() {
        let ip = IpAddress::try_new("192.168.256.1".to_string());
        assert!(matches!(
            ip.unwrap_err(),
            IpAddressError::OctetOutOfRange { position: 3 }
        ));
    }

    #[test]
    fn test_empty_address() {
        let ip = IpAddress::try_new("".to_string());
        assert!(matches!(ip.unwrap_err(), IpAddressError::EmptyAddress));
    }

    #[test]
    fn test_from_str() {
        let ip: Result<IpAddress, _> = "192.168.1.1".parse();
        assert!(ip.is_ok());
    }
}
