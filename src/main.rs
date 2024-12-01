use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{Color, Element, Length, Size, Task, Theme};
use iced_anim::{Animation, Spring, SpringEvent};
use ip_checker_logic::*;

// Main entry point of the application
pub fn main() -> iced::Result {
    // Initialize the iced application with window settings and theme
    iced::application(IpChecker::title, IpChecker::update, IpChecker::view)
        .window(iced::window::Settings {
            min_size: Some(Size {
                width: 300.,
                height: 400.,
            }),
            ..Default::default()
        })
        .window_size(Size {
            width: 300.,
            height: 400.,
        })
        .theme(|state| state.theme.value().clone())
        .run_with(IpChecker::new)
}

// Main application state struct
struct IpChecker {
    calculator: IpCalculator, // Handles IP calculations
    ip: Option<IpModel>,      // Current IP model
    user_input: InputIpModel, // User's input values
    valid: Validator,         // Validation results
    theme: Spring<Theme>,     // Animated theme switcher
}

// Enum defining all possible messages in the application
#[derive(Debug, Clone)]
enum Message {
    GenerateIp,                      // Generate new random IP
    CheckIp,                         // Validate user input
    MaskInput(String),               // Update subnet mask input
    NetworkAddressInput(String),     // Update network address input
    BroadcastAddressInput(String),   // Update broadcast address input
    FirstHostInput(String),          // Update first host input
    LastHostInput(String),           // Update last host input
    PossibleHostsInput(String),      // Update possible hosts input
    ChangeTheme(SpringEvent<Theme>), // Handle theme change animation
}

impl IpChecker {
    // Initialize the application state
    fn new() -> (Self, Task<Message>) {
        let mut calculator = IpCalculator::new();
        (
            Self {
                ip: Some(calculator.random_model()),
                calculator,
                theme: Spring::new(Theme::Dracula),
                user_input: InputIpModel::default(),
                valid: Validator::default(),
            },
            Task::none(),
        )
    }

    // Return the application title
    fn title(&self) -> String {
        String::from("IP Checker")
    }

    // Handle application updates based on received messages
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GenerateIp => {
                self.ip = Some(self.calculator.random_model());
            }
            Message::CheckIp => {
                if let Some(ip) = &self.ip {
                    self.valid = ip.validate(&self.user_input)
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
            Message::ChangeTheme(event) => self.theme.update(event),
        }
        Task::none()
    }

    // Render the application UI
    fn view(&self) -> Element<Message> {
        // Create buttons for generating IP and checking input
        let generate_button = button("Generate IP").on_press(Message::GenerateIp);
        let check_button = button("Check IP").on_press(Message::CheckIp);

        // Create button container with layout
        let button_container = container(
            row![
                generate_button.width(Length::FillPortion(2)),
                check_button.width(Length::FillPortion(2))
            ]
            .spacing(10)
            .width(Length::Fill),
        )
        .width(Length::Fill);

        // Display current IP address
        let ip_text = text(format!(
            "IP: {}{}",
            self.ip
                .as_ref()
                .map(|ip| ip.ip.to_string())
                .unwrap_or("Press Generate IP to get an IP".to_string()),
            self.ip
                .as_ref()
                .map(|ip| ip.prefix.to_string().clone())
                .unwrap_or("".to_string())
        ));

        // Create input fields with validation styling
        let input_fields = column![
            text_input("Enter Subnet Mask", &self.user_input.mask)
                .on_input(Message::MaskInput)
                .style(|theme: &Theme, status| {
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

        // Combine all elements into main content
        let content = column![
            // Theme picker
            pick_list(Theme::ALL, Some(self.theme.target().clone()), |theme| {
                Message::ChangeTheme(theme.into())
            }),
            button_container,
            ip_text,
            input_fields
        ]
        .spacing(10)
        .height(Length::Fill)
        .padding(2);

        // Apply theme animation and return final element
        Animation::new(
            &self.theme,
            container(content)
                .style(move |theme: &Theme| container::Style {
                    background: Some(theme.palette().background.into()),
                    ..Default::default()
                })
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .on_update(Message::ChangeTheme)
        .into()
    }
}
