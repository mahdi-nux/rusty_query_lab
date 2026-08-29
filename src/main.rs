mod database;
mod gui;
mod style;

use crate::gui::{view, update};

fn main() -> iced::Result {
    // iced::run(update, view)
    iced::application(
        gui::State::default,
        update,
        view
    ).theme(|state: &gui::State| style::theme(state.is_dark()))
    .run()
}
