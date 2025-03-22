use bytes::Bytes;
use image::{self, EncodableLayout, open};

use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct HandleRgbaComponents {
    pub width: u32,
    pub height: u32,
    pub pixels: Bytes,
}

impl HandleRgbaComponents {
    pub fn from_rgb_to_greyscale(path: PathBuf) -> Self {
        println!("Path in rgb_to_grayscale {:?}", path);
        let img_buf = open(path).unwrap_or_default().into_rgba8();
        let (width, height) = (img_buf.width(), img_buf.height());

        println!("ImageBuffer Size {}", img_buf.as_bytes().len());
        let rgba_pixels: Vec<u8> = img_buf
            .pixels()
            .flat_map(|p| {
                let r = (p.0[0] as f32 * 0.30).round() as u8;
                let g = (p.0[1] as f32 * 0.59).round() as u8;
                let b = (p.0[2] as f32 * 0.11).round() as u8;

                let g = r + g + b;
                [g, g, g, 255]
            })
            .collect();
        // println!("Pixels: {:?}", rgba_pixels);

        Self {
            width,
            height,
            pixels: Bytes::from(rgba_pixels),
        }
    }

    pub fn greyscale_to_brightness_slice(&self, min_brightness: u8, max_brightness: u8) -> Self {
        let brightness_slice_pixels: Vec<u8> = self
            .pixels
            .chunks(4)
            .flat_map(|rgba| {
                let brightness = (0.299 * rgba[0] as f32
                    + 0.587 * rgba[1] as f32
                    + 0.114 * rgba[2] as f32) as u8;

                // let brightness = ((rgba[0] as f32 + rgba[1] as f32 + rgba[2] as f32) / 3.0).round() as u8 ;

                if brightness >= min_brightness && brightness <= max_brightness {
                    [255, 255, 255, 0]
                } else {
                    [rgba[0], rgba[1], rgba[2], rgba[3]]
                }
            })
            .collect();

        Self {
            width: self.width,
            height: self.height,
            pixels: Bytes::from(brightness_slice_pixels),
        }
    }
}
