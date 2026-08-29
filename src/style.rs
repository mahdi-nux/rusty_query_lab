use iced::{Border, Color};
use iced::widget::container;

pub fn theme(is_dark: bool) -> iced::Theme {
    if is_dark {
        iced::Theme::Dracula
    } else {
        iced::Theme::Light
    }
}

pub fn output(_theme: &iced::Theme, is_dark: bool) -> container::Style {
    container::Style {
        border: Border { 
            color: if is_dark {
                Color::WHITE
            } else {
                Color::BLACK
            }, 
            width: 1.0, 
            radius: 0.0.into() 
        },
        ..Default::default()
    }
}