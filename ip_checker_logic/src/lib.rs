use std::str::FromStr;
mod ip_address;
mod network_address;
mod prefix;
mod subnet_mask;
use ip_address::IpAddress;
use network_address::NetworkAddress;
use prefix::Prefix;
use rand::Rng;
use subnet_mask::SubnetMask;

#[derive(Debug, Clone)]
pub struct IpModel {
    pub ip: IpAddress,
    pub prefix: Prefix,
    pub mask: SubnetMask,
    pub network_address: NetworkAddress,
    pub broadcast_address: IpAddress,
    pub first_host: IpAddress,
    pub last_host: IpAddress,
    pub possible_hosts: Option<u32>,
}
#[derive(Debug, Default, Clone)]
pub struct InputIpModel {
    pub ip: String,
    pub prefix: String,
    pub mask: String,
    pub network_address: String,
    pub broadcast_address: String,
    pub first_host: String,
    pub last_host: String,
    pub possible_hosts: Option<u32>,
}

impl IpModel {
    pub fn validate(&self, other: &InputIpModel) -> Validator {
        Validator {
            mask: self.mask.as_string() == other.mask,
            network_address: self.network_address.as_string() == other.network_address,
            broadcast_address: self.broadcast_address.as_string() == other.broadcast_address,
            first_host: self.first_host.as_string() == other.first_host,
            last_host: self.last_host.as_string() == other.last_host,
            possible_hosts: self.possible_hosts == other.possible_hosts,
        }
    }
}

#[derive(Debug, Default)]
pub struct Validator {
    pub mask: bool,
    pub network_address: bool,
    pub broadcast_address: bool,
    pub first_host: bool,
    pub last_host: bool,
    pub possible_hosts: bool,
}

#[derive(Debug)]
pub struct IpCalculator {
    rng: rand::rngs::ThreadRng,
}

impl IpCalculator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn random_model(&mut self) -> IpModel {
        // Generate random IP and subnet mask
        let octets: [u8; 4] = [
            self.rng.gen_range(111..255),
            self.rng.gen_range(111..255),
            self.rng.gen_range(111..255),
            self.rng.gen_range(111..255),
        ];
        let subnet_mask: u8 = self.rng.gen_range(18..28);

        self.calculate_model(&octets, subnet_mask)
    }

    pub fn calculate_model(&self, octets: &[u8; 4], subnet_mask: u8) -> IpModel {
        // Create IP address and prefix
        let ip = IpAddress::from_str(&format!(
            "{}.{}.{}.{}",
            octets[0], octets[1], octets[2], octets[3]
        ))
        .unwrap();
        let prefix = Prefix::new(subnet_mask).unwrap();

        // Convert IP to u32
        let ip_value = self.octets_to_u32(octets);

        // Get masks from prefix
        let network_mask = prefix.get_network_mask();
        let host_mask = prefix.get_host_mask();

        // Calculate addresses
        let network_value = ip_value & network_mask;
        let broadcast_value = network_value | host_mask;
        let first_host_value = network_value + 1;
        let last_host_value = broadcast_value - 1;

        let model = IpModel {
            ip,
            prefix: prefix.clone(),
            mask: prefix.to_subnet_mask(),
            network_address: NetworkAddress::from_u32(network_value, prefix.value()).unwrap(),
            broadcast_address: IpAddress::from_str(&self.u32_to_ip_string(broadcast_value))
                .unwrap(),
            first_host: IpAddress::from_str(&self.u32_to_ip_string(first_host_value)).unwrap(),
            last_host: IpAddress::from_str(&self.u32_to_ip_string(last_host_value)).unwrap(),
            possible_hosts: Some(prefix.get_max_hosts()),
        };

        #[cfg(debug_assertions)]
        println!("{:?}", model.clone());

        model
    }

    fn octets_to_u32(&self, octets: &[u8; 4]) -> u32 {
        ((octets[0] as u32) << 24)
            | ((octets[1] as u32) << 16)
            | ((octets[2] as u32) << 8)
            | (octets[3] as u32)
    }

    fn u32_to_ip_string(&self, value: u32) -> String {
        format!(
            "{}.{}.{}.{}",
            (value >> 24) & 0xFF,
            (value >> 16) & 0xFF,
            (value >> 8) & 0xFF,
            value & 0xFF
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_calculator() {
        let calculator = IpCalculator::new();
        let octets = [192, 168, 1, 1];
        let prefix_length = 24;

        let model = calculator.calculate_model(&octets, prefix_length);

        assert_eq!(model.ip.as_string(), "192.168.1.1");
        assert_eq!(model.prefix.to_string(), "/24");
        assert_eq!(model.mask.as_string(), "255.255.255.0");
        assert_eq!(model.network_address.as_string(), "192.168.1.0");
        assert_eq!(model.broadcast_address.as_string(), "192.168.1.255");
        assert_eq!(model.first_host.as_string(), "192.168.1.1");
        assert_eq!(model.last_host.as_string(), "192.168.1.254");
        assert_eq!(model.possible_hosts, Some(254));
    }

    #[test]
    fn test_random_model() {
        let mut calculator = IpCalculator::new();
        let model = calculator.random_model();

        // Basic validation that the model is consistent
        assert!(model.possible_hosts.unwrap() > 0);
        assert!(model.first_host.as_string() > model.network_address.as_string());
        assert!(model.last_host.as_string() < model.broadcast_address.as_string());
    }
}
