use iced::{
    executor,
    futures::{channel::mpsc::{channel, Sender, Receiver}, SinkExt},
    widget::{button, column, row, text, text_input},
    Alignment, Application, Command, Executor, Sandbox, Settings, Theme,
};
use std::{io, thread};
// use std::sync::mpsc::channel;

pub struct Login {
    receiver: Receiver<String>,
    username: String,
    password: String,
    hint: String,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    None,
    UsernameChanged(String),
    PasswordChanged(String),
    RecievedMessage(String),
    Login,
    SignUp,
}

async fn recieveMessage(mut tx: Sender<String>) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    tx.send(line);
}

impl Application for Login {
    type Message = LoginMessage;
    type Executor = executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let (mut tx, mut rx) = channel::<String>(500);
        (
            Login {
                receiver: rx,
                username: String::new(),
                password: String::new(),
                hint: String::new(),
            },
            Command::perform(recieveMessage(tx), || LoginMessage::None),
        )
    }

    fn title(&self) -> String {
        String::from("Login")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<LoginMessage> {
        match message {
            Self::Message::UsernameChanged(username) => {
                self.username = username;
            }
            Self::Message::PasswordChanged(password) => {
                self.password = password;
            }
            Self::Message::Login => {
                if self.username.len() > 32 {
                    self.hint = "Username too long".to_string();
                } else if self.password.len() > 32 {
                    self.hint = "Password too long".to_string();
                }
            }
            Self::Message::SignUp => {
                // window::resize<Self::Message>(600, 800);
                // super::signup::SignUp::run(Default::default());
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let username =
            text_input("username", &self.username).on_input(Self::Message::UsernameChanged);
        let password = text_input("password", &self.password)
            .on_input(Self::Message::PasswordChanged)
            .password();
        let hint = text("");
        let login_button = button("Login").on_press(Self::Message::Login);
        let sign_up_button = button("Sign Up").on_press(Self::Message::SignUp);

        column![
            username,
            password,
            hint,
            row![login_button, sign_up_button,]
                .spacing(10)
                .padding(20)
                .align_items(Alignment::Center)
        ]
        .spacing(10)
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    type Theme = Theme;
}

fn main() -> iced::Result {
    Login::run(Settings::default())
}
