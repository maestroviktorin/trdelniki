use iced::{Font, Settings, window};

mod image_processing;
mod ui;
mod hough;

use ui::*;

fn main() -> iced::Result {
    let settings: Settings = iced::settings::Settings {
        default_font: Font::MONOSPACE,
        ..Default::default()
    };

    let window_settings = window::Settings {
        size: iced::Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        position: window::Position::Centered,
        resizable: false,
        ..Default::default()
    };

    iced::application("Trdelniki App", UIState::update, UIState::view)
        .centered()
        .settings(settings)
        .window(window_settings)
        .theme(UIState::theme)
        .run()
}
