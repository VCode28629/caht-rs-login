use iced::{
    executor,
    futures::{
        channel::mpsc::{channel, Receiver, Sender},
        SinkExt,
    },
    widget::{button, column, row, text, text_input},
    Alignment, Application, Command, Executor, Sandbox, Settings, Theme,
};
use std::{io, thread};
// use std::sync::mpsc::channel;

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
            }
            Self::Message::PasswordChanged(password) => {
                self.password = password;
            }
            Self::Message::Login => {
                if self.username.len() > 32 {
                    self.hint = "Username too long".to_string();
                } else if self.password.len() > 32 {
                    self.hint = "Password too long".to_string();
                } else {
                    print!("Login\n{}\n{}", self.username,self.password);
                }
            }
            Self::Message::SignUp => {
                print!("SignUp\n{}\n{}", self.username,self.password);
            }
            Self::Message::RecievedMessage(s) => {
                println!("recieved: {}", s);
            }
        }
        Command::perform(recieve_message(), |s| LoginMessage::RecievedMessage(s))
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
