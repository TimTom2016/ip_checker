use std::str::FromStr;

use thiserror::Error;
#[derive(Debug, Clone)]
pub struct SubnetMask(String);

#[derive(Debug, Error)]
pub enum SubnetMaskError {
    #[error("Invalid number of octets: expected 4, got {0}")]
    InvalidOctetCount(usize),

    #[error("Invalid octet at position {position}: {reason}")]
    InvalidOctet { position: usize, reason: String },

    #[error("Invalid subnet mask: octets must be in descending order")]
    InvalidMaskPattern,

    #[error("Invalid octet value at position {position}: subnet mask octets must be 0, 128, 192, 224, 240, 248, 252, 254, or 255")]
    InvalidOctetValue { position: usize },

    #[error("Empty subnet mask")]
    EmptyMask,
}

impl PartialEq for SubnetMask {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl SubnetMask {
    pub fn new(mask: String) -> Self {
        Self::try_new(mask).unwrap()
    }

    pub fn try_new(mask: String) -> Result<Self, SubnetMaskError> {
        if mask.is_empty() {
            return Err(SubnetMaskError::EmptyMask);
        }

        let octets: Vec<&str> = mask.split('.').collect();
        if octets.len() != 4 {
            return Err(SubnetMaskError::InvalidOctetCount(octets.len()));
        }

        let valid_values = [0, 128, 192, 224, 240, 248, 252, 254, 255];
        let mut previous_octet = 255; // Start with maximum possible value

        for (i, octet) in octets.iter().enumerate() {
            match octet.parse::<u8>() {
                Ok(num) => {
                    // Check if the octet is a valid subnet mask value
                    if !valid_values.contains(&num) {
                        return Err(SubnetMaskError::InvalidOctetValue { position: i + 1 });
                    }

                    // Check if octets are in descending order
                    if num > previous_octet {
                        return Err(SubnetMaskError::InvalidMaskPattern);
                    }
                    previous_octet = num;
                }
                Err(_) => {
                    return Err(SubnetMaskError::InvalidOctet {
                        position: i + 1,
                        reason: format!("'{}' is not a valid number", octet),
                    });
                }
            }
        }

        Ok(Self(mask))
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }

    pub fn to_prefix(&self) -> u8 {
        let mut count = 0;
        for octet in self.0.split('.') {
            let num = octet.parse::<u8>().unwrap();
            count += num.count_ones();
        }
        count as u8
    }
}

impl FromStr for SubnetMask {
    type Err = SubnetMaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_new(s.to_string())
    }
}

impl AsRef<str> for SubnetMask {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SubnetMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_subnet_mask() {
        let mask = SubnetMask::try_new("255.255.255.0".to_string());
        assert!(mask.is_ok());
    }

    #[test]
    fn test_invalid_octet_count() {
        let mask = SubnetMask::try_new("255.255.255".to_string());
        assert!(matches!(
            mask.unwrap_err(),
            SubnetMaskError::InvalidOctetCount(3)
        ));
    }

    #[test]
    fn test_invalid_octet_value() {
        let mask = SubnetMask::try_new("255.255.255.123".to_string());
        assert!(matches!(
            mask.unwrap_err(),
            SubnetMaskError::InvalidOctetValue { position: 4 }
        ));
    }

    #[test]
    fn test_invalid_mask_pattern() {
        let mask = SubnetMask::try_new("255.0.255.0".to_string());
        assert!(matches!(
            mask.unwrap_err(),
            SubnetMaskError::InvalidMaskPattern
        ));
    }

    #[test]
    fn test_empty_mask() {
        let mask = SubnetMask::try_new("".to_string());
        assert!(matches!(mask.unwrap_err(), SubnetMaskError::EmptyMask));
    }

    #[test]
    fn test_prefix_calculation() {
        let mask = SubnetMask::new("255.255.254.0".to_string());
        assert_eq!(mask.to_prefix(), 23);
    }

    #[test]
    fn test_from_str() {
        let mask: Result<SubnetMask, _> = "255.255.255.0".parse();
        assert!(mask.is_ok());
    }
}
