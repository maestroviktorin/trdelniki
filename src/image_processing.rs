use bytes::Bytes;
use image::{self, EncodableLayout, open};

use std::path::PathBuf;

pub fn rgb_to_grayscale(path: PathBuf) -> (u32, u32, Bytes) {
    println!("Path in rgb_to_grayscale {:?}", path);
    let img_buf = open(path).unwrap_or_default().into_rgb8();
    let (width, height) = (img_buf.width(), img_buf.height());

    println!("ImageBuffer Size {}", img_buf.as_bytes().len());
    let rgba_pixels: Vec<u8> = img_buf
        .pixels()
        .flat_map(|p| [p.0[0], p.0[1], p.0[2], 255]) // Add alpha=255
        .collect();
    // println!("Pixels: {:?}", rgba_pixels);

    (width, height, Bytes::from(rgba_pixels))
}
