use iced::widget::{button, column, text, container, Column, vertical_space};
use iced::{Alignment, Element, Sandbox, Settings, Length};


pub fn main() -> iced::Result {
    MainWindow::run(Settings::default())
}

struct MainWindow {}

struct CreateNewSubject{}

#[derive(Debug, Clone, Copy)]
enum Message {
    NewSubjectPressed,
    
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        Self { }
    }

    fn title(&self) -> String {
        String::from("MainWindows - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NewSubjectPressed => {
                println!("New Subject has been pressed!")
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content: Column<'_, Message, _> = column![
            text("Wellcome! Click onto the new subject button to create a new subject to start!"),
            vertical_space(50),
            button("Create new subject").on_press(Message::NewSubjectPressed),
            vertical_space(50),
            "You will be able to create the deck cards selecting one of the deck you created"
        ]
        .padding(100)
        .align_items(Alignment::Center)
        .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl Sandbox for CreateNewSubject {
    type Message = Message;

    fn new() -> Self {
        Self { }
    }

    fn title(&self) -> String {
        String::from("Create new subject - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NewSubjectPressed => {
                todo!()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content: Column<'_, Message, _> = column![
            "Some text here"
            ]
        .padding(100)
        .align_items(Alignment::Center)
        .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}