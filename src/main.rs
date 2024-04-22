#![allow(unused)]
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

use iced::widget::image::Handle;

use iced::{
    alignment::{Horizontal, Vertical},
    font::Family,
    widget::{
        button, column, container, text, text::Shaping, Button, Column, Container, Image, Renderer,
        Row, Text, Theme,
    },
    Alignment, Color, Font, Length, Sandbox, Settings,
};
use iced::{window, Point, Size};

fn main() -> iced::Result {
    MyApp::run(Settings {
        window: window::Settings {
            size: Size {
                width: 210.,
                height: 220.,
            },
            position: window::Position::Specific(Point { x: 900., y: 120. }),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    ButtonPressed,
}

struct MyApp {}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        Self {}
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dracula
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyAppMessage::ButtonPressed => con(),
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

        column![
            text("Ready?").style(Color::from_rgb(1., 0.6, 0.2)),
            container("SSH to ThinkCentre").padding(20),
            container(button("Connect").on_press(MyAppMessage::ButtonPressed))
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            Row::new().push(background) //.push(foreground)
        ]
        .into()
    }
}

fn con() {
    // Connect to the remote SSH server
    let tcp = TcpStream::connect("192.168.1.12:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // Try to authenticate with the first identity in the agent.
    sess.userauth_agent("rag").unwrap();

    // Make sure we succeeded
    assert!(sess.authenticated());

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);

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
