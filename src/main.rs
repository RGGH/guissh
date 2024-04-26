#![allow(unused)]

/// refer to https://github.com/RGGH/iced_tutorial/tree/main
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

use iced::widget::image::Handle;
use iced::{
    alignment::{Horizontal, Vertical},
    font::Family,
    widget::{
        button, column, container, text, text::Shaping, text_input, Button, Column, Container,
        Image, Renderer, Row, Text, Theme,
    },
    window, Alignment, Color, Font, Length, Point, Sandbox, Settings, Size,
};

fn main() -> iced::Result {
    MyApp::run(Settings {
        window: window::Settings {
            size: Size {
                width: 520.,
                height: 420.,
            },
            position: window::Position::Specific(Point { x: 900., y: 120. }),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

pub enum Status {
    Active,
    Hovered,
    Pressed,
    Disabled,
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    ButtonPressed,
    Update6(String),
    Submit6,
}

#[derive(Debug, Clone)]
enum Message {
    ClearStatus,
    TextInputChanged(String),
}

struct MyApp {
    status: String,
    clear_button: button::State,
    text6: String,
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        MyApp {
            status: String::new(),
            clear_button: button::State::new(),
            text6: "".to_string(),
        }
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dracula
    }

    fn title(&self) -> String {
        String::from("Guissh - SSH Client")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyAppMessage::ButtonPressed => {
                //Status::Pressed;
                con(&self.text6)
            }
            MyAppMessage::Update6(s) => {
                self.text6 = s;
            }
            MyAppMessage::Submit6 => todo!(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let image: iced::widget::Image<Handle> = Image::new("resources/ferris.png")
            .width(Length::Fill)
            .height(Length::Fill);

        let background: iced::widget::Container<'_, Self::Message, Theme, Renderer> =
            Container::new(image)
                .width(Length::Fill)
                .height(Length::Fill);
        let status_bar = Text::new(&self.status).width(iced::Length::Fill).size(20);

        let value = "Some text";

        column![
            container("").padding(20).align_x(Horizontal::Center),
            container(button("Connect").on_press(MyAppMessage::ButtonPressed))
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            Row::new().push(background),
            Row::new().padding(30).push(status_bar),
            text("Enter IP address:port number").style(Color::from_rgb(1., 0.6, 0.2)),
            text("eg 192.168.1.12:22").style(Color::from_rgb(1., 0.9, 0.2)),
            text_input("ip address:port", self.text6.as_str())
                .on_input(MyAppMessage::Update6),
        ]
        .into()
    }
}

fn con(tx: &str) {
    // Connect to the remote SSH server
    let tcp = TcpStream::connect(tx).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // Try to authenticate with the first identity in the agent.
    // ssh certificate already on remote machine
    sess.userauth_agent("rag").unwrap();

    // Make sure we succeeded
    assert!(sess.authenticated());

    // linux/unix 'ls'
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    // linux/unix 'uname -a'
    let mut channel = sess.channel_session().unwrap();
    channel.exec("uname -a").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

    let _ = channel.wait_close();

    match channel.exit_status() {
        Ok(0) => println!("{:?}", "Quit ok!"),
        _ => println!("{:?}", "error closing conn"),
    }
}
