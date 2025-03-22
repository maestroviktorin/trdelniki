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
                let r = (p.0[0] as f32 * 0.299).round() as u8;
                let g = (p.0[1] as f32 * 0.587).round() as u8;
                let b = (p.0[2] as f32 * 0.114).round() as u8;

                let g = r + g + b;
                [g, g, g, 255]
            })
            .collect();

        println!("Pixels: {:?}", rgba_pixels.len());

        Self {
            width,
            height,
            pixels: Bytes::from(rgba_pixels),
        }
    }

    pub fn greyscale_to_brightness_slice_keep_bg(
        &self,
        min_brightness: u8,
        max_brightness: u8,
    ) -> Self {
        let brightness_slice_pixels: Vec<u8> = self
            .pixels
            .chunks(4)
            .flat_map(|rgba| {
                let brightness = rgba[0];

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

    pub fn prewitt_filtered(&self) -> Self {
        let width = self.width as usize;
        let height = self.height as usize;

        let grayscale: Vec<u8> = self.pixels.chunks_exact(4).map(|pixel| pixel[0]).collect();

        assert_eq!(grayscale.len(), width * height, "Invalid image dimensions");

        let mut output = vec![0u8; width * height];

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let gx = {
                    let top_left = grayscale[(y - 1) * width + x - 1] as i16;
                    let top_right = grayscale[(y - 1) * width + x + 1] as i16;
                    let mid_left = grayscale[y * width + x - 1] as i16;
                    let mid_right = grayscale[y * width + x + 1] as i16;
                    let bot_left = grayscale[(y + 1) * width + x - 1] as i16;
                    let bot_right = grayscale[(y + 1) * width + x + 1] as i16;

                    (top_right - top_left) + (mid_right - mid_left) + (bot_right - bot_left)
                };

                // Convert to absolute value and clamp to 0-255
                let abs_gx = gx.abs() as u16;
                output[y * width + x] = abs_gx.min(255) as u8;
            }
        }

        let pixels: Bytes = output
            .iter()
            .map(|x| [*x, *x, *x, 255])
            .flat_map(|x| x.to_vec())
            .collect();

        println!("Prewitt {:?}", pixels.len());

        Self {
            width: width as u32,
            height: height as u32,
            pixels,
        }
    }
}
