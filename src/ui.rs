use crate::image_processing::HandleRgbaComponents;

use iced::{
    Element, Task, Theme,
    widget::{
        button, center, column, container,
        image::{self as iced_image, Handle, Viewer},
        row, text, text_input,
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
    min_brightness: String,
    max_brightness: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    UploadImage,
    MinBrightnessChange(String),
    MaxBrightnessChange(String),
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
                    HandleRgbaComponents::from_rgb_to_greyscale(self.image_path.clone().into());
                Task::none()
            }
            Message::MinBrightnessChange(text) => {
                self.min_brightness = text;

                Task::none()
            }
            Message::MaxBrightnessChange(text) => {
                self.max_brightness = text;

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        println!("Path in view {:?}", self.image_path);

        column![
            text("Trdelniki").size(21).width(WINDOW_WIDTH).center(),
            row![
                self.element_image_original(),
                self.element_image_greyscale()
            ],
            row![
                self.element_image_brightness_slice_keep_bg(),
                self.element_image_prewitt_filtered()
            ],
            button(text("Upload File")).on_press(Message::UploadImage),
            text("Параметры яркостного среза:"),
            row![
                text_input(
                    "Минимальная яркость (по умолчанию 0)",
                    self.min_brightness.as_ref()
                )
                .on_input(Message::MinBrightnessChange),
                text_input(
                    "Максимальная яркость (по умолчанию 255)",
                    self.max_brightness.as_ref()
                )
                .on_input(Message::MaxBrightnessChange)
            ],
        ]
        .spacing(10)
        .padding(5)
        .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::GruvboxLight
    }

    fn element_image_original(&self) -> Element<Message> {
        column![
            center(iced::widget::image(self.image_path.clone())).style(container::bordered_box),
            text("Оригинал изображения")
        ]
        .into()
    }

    fn element_image_greyscale(&self) -> Element<Message> {
        column![
            center(self.image_greyscale(iced_image::FilterMethod::Nearest))
                .style(container::bordered_box),
            text("Изображение в оттенках серого")
        ]
        .into()
    }

    fn element_image_brightness_slice_keep_bg(&self) -> Element<Message> {
        column![
            center(self.image_brightness_slice_keep_bg(iced_image::FilterMethod::Nearest))
                .style(container::bordered_box),
            text("Яркостный срез с сохранением фона")
        ]
        .into()
    }

    fn element_image_prewitt_filtered(&self) -> Element<Message> {
        column![
            center(self.image_prewitt_filtered(iced_image::FilterMethod::Nearest))
                .style(container::bordered_box),
            text("Изображение под x-Prewitt-фильтром")
        ]
        .into()
    }

    fn image_greyscale<'a>(&self, filter_method: iced_image::FilterMethod) -> Viewer<Handle> {
        let handle = Handle::from_rgba(
            self.handle_rgba_components.width,
            self.handle_rgba_components.height,
            self.handle_rgba_components.pixels.clone(),
        );
        println!("Handle {:#?}", handle);

        let img = iced::widget::image::viewer(handle)
            .width(WINDOW_WIDTH / 3.0)
            .height(VIEWER_HEIGHT / 3.0)
            .filter_method(filter_method);

        img
    }

    fn image_brightness_slice_keep_bg<'a>(
        &self,
        filter_method: iced_image::FilterMethod,
    ) -> Viewer<Handle> {
        let handle = Handle::from_rgba(
            self.handle_rgba_components.width,
            self.handle_rgba_components.height,
            self.handle_rgba_components
                .greyscale_to_brightness_slice_keep_bg(
                    self.min_brightness.parse().unwrap_or_default(),
                    self.max_brightness.parse().unwrap_or_default(),
                )
                .pixels
                .clone(),
        );
        println!("Handle {:#?}", handle);

        let img = iced::widget::image::viewer(handle)
            .width(WINDOW_WIDTH / 3.0)
            .height(VIEWER_HEIGHT / 3.0)
            .filter_method(filter_method);

        img
    }

    fn image_prewitt_filtered<'a>(
        &self,
        filter_method: iced_image::FilterMethod,
    ) -> Viewer<Handle> {
        let handle = Handle::from_rgba(
            self.handle_rgba_components.width,
            self.handle_rgba_components.height,
            self.handle_rgba_components
                .prewitt_filtered()
                .pixels
                .clone(),
        );
        println!("Handle {:#?}", handle);

        let img = iced::widget::image::viewer(handle)
            .width(WINDOW_WIDTH / 3.0)
            .height(VIEWER_HEIGHT / 3.0)
            .filter_method(filter_method);

        img
    }
}
