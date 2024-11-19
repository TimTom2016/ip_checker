use iced::widget::text_input::Catalog;
use iced::widget::{button, column, container, row, text, text_input};
use iced::Settings;
use iced::{Color, Element, Length, Size, Task, Theme};
use rand::{thread_rng, Rng};

pub fn main() -> iced::Result {
    iced::application(IpChecker::title, IpChecker::update, IpChecker::view)
        .window(iced::window::Settings {
            min_size: Some(Size {
                width: 200.,
                height: 400.,
            }),
            ..Default::default()
        })
        .window_size(Size {
            width: 200.,
            height: 400.,
        })
        .run_with(IpChecker::new)
}

struct IpChecker {
    ip: Option<IpModel>,
    user_input: IpModel,
    valid: Validator,
}

#[derive(Debug, Clone)]
enum Message {
    GenerateIp,
    CheckIp,
    MaskInput(String),
    NetworkAddressInput(String),
    BroadcastAddressInput(String),
    FirstHostInput(String),
    LastHostInput(String),
    PossibleHostsInput(String),
}

#[derive(Debug, Default, Clone)]
struct IpModel {
    ip: String,
    subnet_mask: u8,
    prefix: String,
    mask: String,
    binary_address: String,
    network_address: String,
    broadcast_address: String,
    first_host: String,
    last_host: String,
    possible_hosts: Option<u32>,
}

#[derive(Debug, Default)]
struct Validator {
    mask: bool,
    network_address: bool,
    broadcast_address: bool,
    first_host: bool,
    last_host: bool,
    possible_hosts: bool,
}

impl IpChecker {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                ip: Some(ip_calculations()),
                user_input: IpModel::default(),
                valid: Validator::default(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("IP Checker")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GenerateIp => {
                self.ip = Some(ip_calculations());
            }
            Message::CheckIp => {
                if let Some(ip) = &self.ip {
                    self.valid = Validator {
                        mask: ip.mask == self.user_input.mask,
                        network_address: ip.network_address == self.user_input.network_address,
                        broadcast_address: ip.broadcast_address
                            == self.user_input.broadcast_address,
                        first_host: ip.first_host == self.user_input.first_host,
                        last_host: ip.last_host == self.user_input.last_host,
                        possible_hosts: ip.possible_hosts == self.user_input.possible_hosts,
                    };
                }
            }
            Message::MaskInput(value) => self.user_input.mask = value,
            Message::NetworkAddressInput(value) => self.user_input.network_address = value,
            Message::BroadcastAddressInput(value) => self.user_input.broadcast_address = value,
            Message::FirstHostInput(value) => self.user_input.first_host = value,
            Message::LastHostInput(value) => self.user_input.last_host = value,
            Message::PossibleHostsInput(value) => {
                self.user_input.possible_hosts = if value.is_empty() {
                    None
                } else {
                    value.parse().ok()
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let generate_button = button("Generate IP").on_press(Message::GenerateIp);

        let ip_text = text(format!(
            "IP: {}{}",
            self.ip
                .as_ref()
                .map(|ip| &ip.ip)
                .unwrap_or(&"Press Generate IP to get an IP".to_string()),
            self.ip
                .as_ref()
                .map(|ip| &ip.prefix)
                .unwrap_or(&"".to_string())
        ));

        let input_fields = column![
            text_input("Enter Subnet Mask", &self.user_input.mask)
                .on_input(Message::MaskInput)
                .style(|theme: &Theme, status| {
                    let palette = theme.extended_palette();
                    let mut style = text_input::default(theme, status);
                    style.border.color = if self.valid.mask {
                        Color::from_rgb8(0, 255, 0)
                    } else {
                        Color::from_rgb8(255, 0, 0)
                    };
                    style.border.width = 2.0;
                    style
                })
                .padding(5),
            text_input("Network Address", &self.user_input.network_address)
                .on_input(Message::NetworkAddressInput)
                .style(|theme: &Theme, status| {
                    let palette = theme.extended_palette();
                    let mut style = text_input::default(theme, status);
                    style.border.color = if self.valid.network_address {
                        Color::from_rgb8(0, 255, 0)
                    } else {
                        Color::from_rgb8(255, 0, 0)
                    };
                    style.border.width = 2.0;
                    style
                })
                .padding(5),
            text_input("Broadcast Address", &self.user_input.broadcast_address)
                .on_input(Message::BroadcastAddressInput)
                .style(|theme: &Theme, status| {
                    let palette = theme.extended_palette();
                    let mut style = text_input::default(theme, status);
                    style.border.color = if self.valid.broadcast_address {
                        Color::from_rgb8(0, 255, 0)
                    } else {
                        Color::from_rgb8(255, 0, 0)
                    };
                    style.border.width = 2.0;
                    style
                })
                .padding(5),
            text_input("First Host", &self.user_input.first_host)
                .on_input(Message::FirstHostInput)
                .style(|theme: &Theme, status| {
                    let palette = theme.extended_palette();
                    let mut style = text_input::default(theme, status);
                    style.border.color = if self.valid.first_host {
                        Color::from_rgb8(0, 255, 0)
                    } else {
                        Color::from_rgb8(255, 0, 0)
                    };
                    style.border.width = 2.0;
                    style
                })
                .padding(5),
            text_input("Last Host", &self.user_input.last_host)
                .on_input(Message::LastHostInput)
                .style(|theme: &Theme, status| {
                    let palette = theme.extended_palette();
                    let mut style = text_input::default(theme, status);
                    style.border.color = if self.valid.last_host {
                        Color::from_rgb8(0, 255, 0)
                    } else {
                        Color::from_rgb8(255, 0, 0)
                    };
                    style.border.width = 2.0;
                    style
                })
                .padding(5),
            text_input(
                "Possible Hosts",
                &self
                    .user_input
                    .possible_hosts
                    .map_or(String::new(), |n| n.to_string())
            )
            .on_input(Message::PossibleHostsInput)
            .style(|theme: &Theme, status| {
                let palette = theme.extended_palette();
                let mut style = text_input::default(theme, status);
                style.border.color = if self.valid.possible_hosts {
                    Color::from_rgb8(0, 255, 0)
                } else {
                    Color::from_rgb8(255, 0, 0)
                };
                style.border.width = 2.0;
                style
            })
            .padding(5),
        ]
        .spacing(5);

        let check_button = button("Check IP").on_press(Message::CheckIp);

        let content = column![generate_button, ip_text, input_fields, check_button]
            .spacing(10)
            .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn ip_calculations() -> IpModel {
    let mut rng = thread_rng();
    let octets: [u8; 4] = [
        rng.gen_range(111..255),
        rng.gen_range(111..255),
        rng.gen_range(111..255),
        rng.gen_range(111..255),
    ];
    let subnet_mask: u8 = rng.gen_range(18..28);
    let prefix = format!("/{}", subnet_mask);
    let ip = format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3]);
    let mut binary_mask = "1".repeat(subnet_mask as usize) + &"0".repeat(32 - subnet_mask as usize);
    binary_mask = add_dots(&binary_mask);
    let binary_address = format!(
        "{:08b}.{:08b}.{:08b}.{:08b}",
        octets[0], octets[1], octets[2], octets[3]
    );

    // calculate network address
    let mut network_address = String::new();
    for (i, c) in binary_address.chars().enumerate() {
        if c == '.' {
            network_address.push('.');
        } else {
            network_address.push_str(
                &((c.to_digit(10).unwrap()
                    & binary_mask.chars().nth(i).unwrap().to_digit(10).unwrap())
                .to_string()),
            );
        }
    }

    // calculate broadcast address
    let mut broadcast_address = String::new();
    for (i, c) in binary_address.chars().enumerate() {
        if i >= subnet_mask as usize {
            if c == '.' {
                broadcast_address.push('.');
            } else {
                broadcast_address.push('1');
            }
        } else {
            broadcast_address.push(c);
        }
    }
    // calculate broadcast address
    let mut broadcast_address = String::new();
    for (i, c) in network_address.chars().enumerate() {
        if i >= (subnet_mask + subnet_mask / 8) as usize {
            if c == '.' {
                broadcast_address.push('.');
            } else {
                broadcast_address.push('1');
            }
        } else {
            broadcast_address.push(c);
        }
    }

    // calculate first host address
    let mut first_host = network_address.clone();
    let first_host_int = u32::from_str_radix(&first_host.replace(".", ""), 2).unwrap() + 1;
    first_host = format!("{:032b}", first_host_int);
    first_host = add_dots(&first_host);
    // calculate last host address
    let mut last_host = broadcast_address.clone();
    let last_host_int = u32::from_str_radix(&last_host.replace(".", ""), 2).unwrap() - 1;
    last_host = format!("{:032b}", last_host_int);
    last_host = add_dots(&last_host);

    broadcast_address = binary_dotted_to_decimal(&broadcast_address);

    first_host = binary_dotted_to_decimal(&first_host);
    binary_mask = binary_dotted_to_decimal(&binary_mask);
    last_host = binary_dotted_to_decimal(&last_host);
    network_address = binary_dotted_to_decimal(&network_address);
    // calculate number of possible hosts
    let number_of_hosts = if subnet_mask < 31 {
        2u32.pow(32 - subnet_mask as u32) - 2
    } else {
        0
    };
    let model = IpModel {
        ip,
        subnet_mask,
        prefix,
        mask: binary_mask,
        binary_address,
        network_address,
        broadcast_address,
        first_host,
        last_host,
        possible_hosts: Some(number_of_hosts),
    };
    #[cfg(debug_assertions)]
    println!("{:?}", model.clone());
    return model;
}

fn add_dots(binary_string: &str) -> String {
    let mut result = String::new();
    for (i, c) in binary_string.chars().enumerate() {
        if i % 8 == 0 && i != 0 {
            result.push('.');
        }
        result.push(c);
    }
    result
}

fn binary_dotted_to_decimal(binary_dotted: &str) -> String {
    let mut result = String::new();
    for octet in binary_dotted.split('.') {
        result.push_str(&u8::from_str_radix(octet, 2).unwrap().to_string());
        result.push('.');
    }
    result.pop();
    result
}
