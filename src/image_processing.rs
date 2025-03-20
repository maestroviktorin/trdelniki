use bytes::Bytes;
use image::{self, EncodableLayout, open};

use std::path::PathBuf;

pub fn rgb_to_grayscale(path: PathBuf) -> (u32, u32, Bytes) {
    println!("Path in rgb_to_grayscale {:?}", path);
    let img_buf = open(path).unwrap_or_default().into_rgb8();

    println!("ImageBuffer Size {}", img_buf.as_bytes().len());

    let width = img_buf.width();
    let height = img_buf.height();
    let pixels = img_buf.into_vec();
    println!("Pixels: {:?}", pixels);

    (width, height, pixels.into())
}
