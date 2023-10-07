use iced::{
    executor,
    widget::{column, container, text},
    Application, Command, Length, Theme,
};

#[derive(Default)]
pub struct GameOfLife {
    speed: usize,
}

#[derive(Debug)]
pub enum Message {}

impl Application for GameOfLife {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                speed: 5,
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Conway's Game of Life".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = column![text("This is something")];
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
