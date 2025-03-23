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
        if self.width == 0 || self.height == 0 {
            return self.clone();
        }

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

    pub fn _hough_transform(
        &self,
        theta_step: f32,
        rho_step: f32,
        edge_threshold: f32,
        accumulator_threshold: usize,
    ) -> Vec<(f32, f32)> {
        let width = self.width as usize;
        let height = self.height as usize;

        // Extract grayscale intensities (R component of each RGBA pixel)
        let intensities: Vec<u8> = self.pixels.chunks_exact(4).map(|c| c[0]).collect();

        // Edge detection using Sobel operators
        let mut edges = vec![false; width * height];
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                // Sobel Gx kernel
                let gx = (-1.0 * intensities[(y - 1) * width + (x - 1)] as f32)
                    + (-2.0 * intensities[(y) * width + (x - 1)] as f32)
                    + (-1.0 * intensities[(y + 1) * width + (x - 1)] as f32)
                    + (1.0 * intensities[(y - 1) * width + (x + 1)] as f32)
                    + (2.0 * intensities[(y) * width + (x + 1)] as f32)
                    + (1.0 * intensities[(y + 1) * width + (x + 1)] as f32);

                // Sobel Gy kernel
                let gy = (-1.0 * intensities[(y - 1) * width + (x - 1)] as f32)
                    + (-2.0 * intensities[(y - 1) * width + x] as f32)
                    + (-1.0 * intensities[(y - 1) * width + (x + 1)] as f32)
                    + (1.0 * intensities[(y + 1) * width + (x - 1)] as f32)
                    + (2.0 * intensities[(y + 1) * width + x] as f32)
                    + (1.0 * intensities[(y + 1) * width + (x + 1)] as f32);

                let magnitude = (gx.powi(2) + gy.powi(2)).sqrt();
                if magnitude >= edge_threshold {
                    edges[y * width + x] = true;
                }
            }
        }

        // Collect edge coordinates
        let edge_coordinates: Vec<(usize, usize)> = edges
            .iter()
            .enumerate()
            .filter_map(|(i, &is_edge)| {
                if is_edge {
                    Some((i % width, i / width))
                } else {
                    None
                }
            })
            .collect();

        // Hough Transform parameters
        let theta_step_rad = theta_step.to_radians();
        let theta_bins = (180.0 / theta_step).ceil() as usize;
        let thetas: Vec<f32> = (0..theta_bins).map(|i| i as f32 * theta_step_rad).collect();

        let max_rho = ((width as f32).powi(2) + (height as f32).powi(2)).sqrt();
        let rho_bins = ((2.0 * max_rho) / rho_step).ceil() as usize;

        // Initialize and populate accumulator
        let mut accumulator = vec![0; theta_bins * rho_bins];
        for &(x, y) in &edge_coordinates {
            let x = x as f32;
            let y = y as f32;
            for (theta_idx, &theta) in thetas.iter().enumerate() {
                let rho = x * theta.cos() + y * theta.sin();
                let rho_discretized = ((rho + max_rho) / rho_step).round() as isize;
                if rho_discretized >= 0 && rho_discretized < rho_bins as isize {
                    let acc_idx = theta_idx * rho_bins + rho_discretized as usize;
                    accumulator[acc_idx] += 1;
                }
            }
        }

        // Extract lines from accumulator
        let mut lines = Vec::new();
        for theta_idx in 0..theta_bins {
            for rho_bin in 0..rho_bins {
                let acc_idx = theta_idx * rho_bins + rho_bin;
                if accumulator[acc_idx] >= accumulator_threshold {
                    let theta = theta_idx as f32 * theta_step_rad;
                    let rho = (rho_bin as f32 * rho_step) - max_rho;
                    lines.push((rho, theta));
                }
            }
        }

        lines
    }

    // Helper function to convert Hough lines to line segments
    pub fn _hough_lines_to_segments(&self, lines: &[(f32, f32)]) -> Vec<((f32, f32), (f32, f32))> {
        let width = self.width as f32;
        let height = self.height as f32;
        let mut segments = Vec::new();

        for &(rho, theta) in lines {
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            // Calculate intersection points with image boundaries
            let mut points = Vec::new();

            // Check intersection with top (y=0) and bottom (y=height) boundaries
            if sin_theta.abs() > 1e-6 {
                // Intersection with top edge (y = 0)
                let x = (rho - 0.0 * sin_theta) / cos_theta;
                if x >= 0.0 && x <= width {
                    points.push((x, 0.0));
                }

                // Intersection with bottom edge (y = height)
                let x = (rho - height * sin_theta) / cos_theta;
                if x >= 0.0 && x <= width {
                    points.push((x, height));
                }
            }

            // Check intersection with left (x=0) and right (x=width) boundaries
            if cos_theta.abs() > 1e-6 {
                // Intersection with left edge (x = 0)
                let y = (rho - 0.0 * cos_theta) / sin_theta;
                if y >= 0.0 && y <= height {
                    points.push((0.0, y));
                }

                // Intersection with right edge (x = width)
                let y = (rho - width * cos_theta) / sin_theta;
                if y >= 0.0 && y <= height {
                    points.push((width, y));
                }
            }

            // If we have two points, create a line segment
            if points.len() >= 2 {
                // Take the first two distinct points
                let p1 = points[0];
                let p2 = points[1];
                segments.push((p1, p2));
            }
        }

        segments
    }
}
