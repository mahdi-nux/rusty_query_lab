use iced::{Border, Color};
use iced::widget::container;

pub fn output(_theme: &iced::Theme) -> container::Style {
    container::Style {
        border: Border { 
            color: Color::BLACK, 
            width: 1.0, 
            radius: 0.0.into() 
        },
        ..Default::default()
    }
}