use crate::image_processing;

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
pub static WINDOW_HEIGHT: f32 = 600.0;

#[derive(Debug, Default)]
pub struct UIState {
    image: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    UploadImage,
}

impl UIState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UploadImage => {
                self.image = FileDialog::new()
                    .add_filter("image", &["png", "jpg", "jpeg"])
                    .pick_file()
                    .unwrap()
                    .as_os_str()
                    .to_owned()
                    .into_string()
                    .unwrap();
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        println!("Path in view {:?}", self.image);

        column!(
            text("Trdelniki").size(21).width(WINDOW_WIDTH).center(),
            self._test_image(iced_image::FilterMethod::Linear),
            text(format!("{:#?}", self.image)),
            button(text("Upload File")).on_press(Message::UploadImage)
        )
        .spacing(10)
        .padding(5)
        .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::GruvboxLight
    }

    fn _test_image<'a>(&self, _filter_method: iced_image::FilterMethod) -> Container<'a, Message> {
        // All the bytes go well until there.
        let (w, h, pixels) = image_processing::rgb_to_grayscale(PathBuf::from(self.image.clone()));

        //println!("{:?}", img_bytes);

        // Then bytes got broken right here.
        let handle = Handle::from_rgba(w, h, pixels);
        println!("Handle {:#?}", handle);

        let img = iced::widget::image(handle);

        center(img).style(container::bordered_box)
    }

    // fn _selected_image<'a>(
    //     &self,
    //     filter_method: iced_image::FilterMethod,
    // ) -> Container<'a, Message> {
    //     center(
    //         iced::widget::image(Handle::from_bytes(image_processing::rgb_to_grayscale(
    //             PathBuf::from(self.image.clone()),
    //         )))
    //         .filter_method(filter_method),
    //     )
    //     .style(container::bordered_box)
    // }
}
