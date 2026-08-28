mod database;
mod gui;

use crate::gui::{view, update};

fn main() -> iced::Result {
    iced::run(update, view)
}
