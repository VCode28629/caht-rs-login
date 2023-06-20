use iced::{
    executor,
    widget::{button, column, row, text, text_input},
    window, Alignment, Application, Color, Command, Element, Settings, Theme,
};
use std::{io, process::exit};

#[derive(Default)]
pub struct Login {
    username: String,
    password: String,
    hint: String,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UsernameChanged(String),
    PasswordChanged(String),
    RecievedMessage(String),
    Login,
    SignUp,
}

async fn recieve_message() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

impl Drop for Login {
    fn drop(&mut self) {
        print!("Exit\n");
    }
}
impl Login {
    fn handle_message(&mut self, message: String) {
        let message = message.trim_end();
        match message {
            "EXIST" => {
                self.hint = "User Exists".to_string();
            }
            "OK" => {
                exit(0);
            }
            "WRONG" => {
                self.hint = "Wrong Username or Password".to_string();
            }
            _ => {
                eprintln!("unreachable message: {message}");
                unreachable!()
            }
        }
    }
}

impl Application for Login {
    type Message = LoginMessage;
    type Executor = executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Default::default(),
            Command::perform(recieve_message(), |s| LoginMessage::RecievedMessage(s)),
        )
    }

    fn title(&self) -> String {
        String::from("Login")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<LoginMessage> {
        match message {
            Self::Message::UsernameChanged(username) => {
                self.username = username;
                Command::none()
            }
            Self::Message::PasswordChanged(password) => {
                self.password = password;
                Command::none()
            }
            Self::Message::Login => {
                if self.username.len() > 32 {
                    self.hint = "Username too long".to_string();
                } else if self.password.len() > 32 {
                    self.hint = "Password too long".to_string();
                } else if self.username.is_empty() {
                    self.hint = "Please enter a username".to_string();
                } else if self.password.is_empty() {
                    self.hint = "Please enter your password".to_string();
                } else {
                    print!("Login\n{}\n{}\n", self.username, self.password);
                }
                Command::none()
            }
            Self::Message::SignUp => {
                if self.username.len() > 32 {
                    self.hint = "Username too long".to_string();
                } else if self.password.len() > 32 {
                    self.hint = "Password too long".to_string();
                } else if self.username.is_empty() {
                    self.hint = "Please enter a username".to_string();
                } else if self.password.is_empty() {
                    self.hint = "Please enter your password".to_string();
                } else {
                    print!("SignUp\n{}\n{}\n", self.username, self.password);
                }
                Command::none()
            }
            Self::Message::RecievedMessage(s) => {
                self.handle_message(s);
                // eprint!("recieved: {}\n", s);
                Command::perform(recieve_message(), |s| LoginMessage::RecievedMessage(s))

            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let username =
            text_input("username", &self.username).on_input(Self::Message::UsernameChanged);
        let password = text_input("password", &self.password)
            .on_input(Self::Message::PasswordChanged)
            .password();
        let hint = text(&self.hint);
        let login_button = button("Login").on_press(Self::Message::Login).height(38);
        let sign_up_button = button("Sign Up").on_press(Self::Message::SignUp).height(38);
        let content: Element<_> = column![
            column![username, password].spacing(10),
            hint,
            row![login_button, sign_up_button,]
                .spacing(10)
                .padding(10)
                .align_items(Alignment::Center)
        ]
        // .spacing(10)
        .padding(20)
        .align_items(Alignment::Center)
        .into();
        content
        // .explain(Color::BLACK)
    }

    type Theme = Theme;
}

fn main() -> iced::Result {
    // let font = font::load();
    Login::run(Settings {
        window: window::Settings {
            size: (320, 180),
            resizable: false,
            ..Default::default()
        },
        // default_font: Some(include_bytes!("../resources/fonts/simsun.ttc")),
        ..Default::default()
    })
}
