use iced::alignment::{Alignment};
use iced::event::{Event};
use iced::executor;
use iced::theme::{self, Theme, Palette, Custom};
use iced::widget::{ Column, Row, container, Text, };
use iced::{
    Application, Color, Command, Element, Length, Settings, Subscription,
};

use wordclock::{MAX_COLUMNS, MAX_ROWS, CH_BERN_GRID};


pub fn main() -> iced::Result {
    ClockWordArea::run(Settings {
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
}

struct ClockWordArea {
    now: time::OffsetDateTime,
}


impl Application for ClockWordArea {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    fn theme(&self) -> Self::Theme {
        Theme::Custom(Box::new( Custom::new (Palette {
            background : Color::BLACK,
            text : Color { r: 0.0,  g: 1.0,  b: 0.0,  a: 0.5,},
            primary :Color { r: 0.0,  g: 1.0,  b: 0.0,  a: 1.0 },
            success : Color { r: 0.0,  g: 1.0,  b: 0.0,  a: 1.0,},
            danger : Color { r: 0.0,  g: 1.0,  b: 0.0,  a: 1.0,},
        } )))
    }

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            ClockWordArea {
                now: time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()), },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Word Clock - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(500)).map(|_| {
            Message::Tick(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        })
    }

    fn view(&self) -> Element<Message> {

        let mut col = Column::new()
        .spacing(10)
        .padding(10)
        .align_items(Alignment::Center);

        for col_index in 0..MAX_COLUMNS {
            let mut row = Row::new()
                .spacing(10)
                .padding(10)
                .align_items(Alignment::Center);
            for row_index in 0..MAX_ROWS {
                let r = row.push(
                    Text::new(CH_BERN_GRID[col_index * MAX_ROWS + row_index])
                        .height(40)
                        .width(40)
                        .size(32)
                        // .style(theme::Text::Color( Color { r: 1.0,  g: 0.0,  b: 0.0,  a: 1.0,}))
                    );

                row = r;
            }
            let c = col.push(row);
            col = c;
        }

        container(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

}