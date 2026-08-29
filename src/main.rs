mod database;
mod gui;
mod style;

use crate::gui::{view, update};

fn main() -> iced::Result {
    iced::run(update, view)
}
