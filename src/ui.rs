use crate::image_processing::HandleRgbaComponents;

use iced::{
    Element, Task, Theme,
    widget::{
        Container, button, center, column, container,
        image::{self as iced_image, Handle},
        text,
    },
};
use rfd::FileDialog;

use std::path::PathBuf;

pub static WINDOW_WIDTH: f32 = 1000.0;
pub static WINDOW_HEIGHT: f32 = 650.0;
static _VIEWER_WIDTH: f32 = 800.0;
static VIEWER_HEIGHT: f32 = 600.0;

#[derive(Debug, Default)]
pub struct UIState {
    image_path: PathBuf,
    handle_rgba_components: HandleRgbaComponents,
}

#[derive(Debug, Clone)]
pub enum Message {
    UploadImage,
}

impl UIState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UploadImage => {
                self.image_path = FileDialog::new()
                    .add_filter("image", &["png", "jpg", "jpeg"])
                    .pick_file()
                    .unwrap_or_default();
                self.handle_rgba_components =
                    HandleRgbaComponents::rgb_to_greyscale(self.image_path.clone().into());

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        println!("Path in view {:?}", self.image_path);

        column!(
            text("Trdelniki").size(21).width(WINDOW_WIDTH).center(),
            self.greyscale_image(iced_image::FilterMethod::Nearest),
            text(format!("{:#?}", self.image_path)),
            button(text("Upload File")).on_press(Message::UploadImage)
        )
        .spacing(10)
        .padding(5)
        .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::GruvboxLight
    }

    fn greyscale_image<'a>(
        &self,
        filter_method: iced_image::FilterMethod,
    ) -> Container<'a, Message> {
        let handle = Handle::from_rgba(
            self.handle_rgba_components.width,
            self.handle_rgba_components.height,
            self.handle_rgba_components.pixels.clone(),
        );
        println!("Handle {:#?}", handle);

        let img = iced::widget::image::viewer(handle)
            .width(WINDOW_WIDTH)
            .height(VIEWER_HEIGHT)
            .filter_method(filter_method);

        center(img).style(container::bordered_box)
    }
}
