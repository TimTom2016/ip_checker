use std::str::FromStr;

use thiserror::Error;

use crate::subnet_mask::SubnetMask;

#[derive(Debug, Clone)]
pub struct Prefix(u8);

#[derive(Debug, Error)]
pub enum PrefixError {
    #[error("Invalid prefix length: must be between 1 and 32, got {0}")]
    InvalidLength(u8),

    #[error("Failed to parse prefix: {0}")]
    ParseError(String),
}

impl PartialEq for Prefix {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Prefix {
    pub fn new(prefix: u8) -> Result<Self, PrefixError> {
        if prefix < 1 || prefix > 32 {
            return Err(PrefixError::InvalidLength(prefix));
        }
        Ok(Self(prefix))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn to_subnet_mask(&self) -> SubnetMask {
        let mask_value = !0u32 << (32 - self.0);
        let mask_str = format!(
            "{}.{}.{}.{}",
            (mask_value >> 24) & 0xFF,
            (mask_value >> 16) & 0xFF,
            (mask_value >> 8) & 0xFF,
            mask_value & 0xFF
        );
        SubnetMask::new(mask_str)
    }

    pub fn from_subnet_mask(mask: &SubnetMask) -> Result<Self, PrefixError> {
        let mut count = 0;
        let mut consecutive = true;

        // Convert mask to binary string
        let binary = mask
            .as_string()
            .split('.')
            .map(|octet| {
                let num = octet.parse::<u8>().unwrap();
                format!("{:08b}", num)
            })
            .collect::<String>();

        // Count consecutive 1s
        for bit in binary.chars() {
            if bit == '1' {
                if !consecutive {
                    return Err(PrefixError::ParseError(
                        "Invalid subnet mask: non-consecutive 1s".to_string(),
                    ));
                }
                count += 1;
            } else {
                consecutive = false;
            }
        }

        Self::new(count)
    }
}

impl FromStr for Prefix {
    type Err = PrefixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(prefix_str) = s.strip_prefix('/') {
            match prefix_str.parse::<u8>() {
                Ok(prefix) => Self::new(prefix),
                Err(_) => Err(PrefixError::ParseError(
                    "Failed to parse prefix number".to_string(),
                )),
            }
        } else {
            Err(PrefixError::ParseError(
                "Prefix must start with '/'".to_string(),
            ))
        }
    }
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}", self.0)
    }
}

impl Default for Prefix {
    fn default() -> Self {
        Self(24) // Common default of /24
    }
}

// Helper methods for network calculations
impl Prefix {
    pub fn get_network_mask(&self) -> u32 {
        !0u32 << (32 - self.0)
    }

    pub fn get_host_mask(&self) -> u32 {
        !self.get_network_mask()
    }

    pub fn get_max_hosts(&self) -> u32 {
        if self.0 >= 31 {
            0
        } else {
            (1u32 << (32 - self.0)) - 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_prefix() {
        let prefix = Prefix::new(24);
        assert!(prefix.is_ok());
        assert_eq!(prefix.unwrap().value(), 24);
    }

    #[test]
    fn test_invalid_prefix_length() {
        let prefix = Prefix::new(33);
        assert!(matches!(
            prefix.unwrap_err(),
            PrefixError::InvalidLength(33)
        ));
    }

    #[test]
    fn test_from_str_valid() {
        let prefix: Result<Prefix, _> = "/24".parse();
        assert!(prefix.is_ok());
        assert_eq!(prefix.unwrap().value(), 24);
    }

    #[test]
    fn test_from_str_invalid() {
        let prefix: Result<Prefix, _> = "24".parse();
        assert!(matches!(prefix.unwrap_err(), PrefixError::ParseError(_)));
    }

    #[test]
    fn test_to_subnet_mask() {
        let prefix = Prefix::new(24).unwrap();
        let mask = prefix.to_subnet_mask();
        assert_eq!(mask.as_string(), "255.255.255.0");
    }

    #[test]
    fn test_max_hosts() {
        let prefix = Prefix::new(24).unwrap();
        assert_eq!(prefix.get_max_hosts(), 254); // 256 - 2 for network and broadcast
    }

    #[test]
    fn test_network_mask() {
        let prefix = Prefix::new(24).unwrap();
        assert_eq!(prefix.get_network_mask(), 0xFFFFFF00);
    }

    #[test]
    fn test_host_mask() {
        let prefix = Prefix::new(24).unwrap();
        assert_eq!(prefix.get_host_mask(), 0x000000FF);
    }

    #[test]
    fn test_from_subnet_mask() {
        let mask = SubnetMask::new("255.255.255.0".to_string());
        let prefix = Prefix::from_subnet_mask(&mask).unwrap();
        assert_eq!(prefix.value(), 24);
    }
}
