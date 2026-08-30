mod database;
mod gui;
mod style;
mod setting;

use crate::gui::{view, update};

fn main() -> iced::Result {
    iced::application(
        gui::State::default,
        update,
        view
    ).theme(|state: &gui::State| style::theme(state.is_dark()))
    .run()
}
