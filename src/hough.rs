use bytes::Bytes;
use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_line_segment_mut;
use nalgebra as na;

use std::f64::consts::PI;

use crate::image_processing::HandleRgbaComponents;

impl HandleRgbaComponents {
    pub fn hough_transform(
        &self,
        theta_scale_factor: u32,
        rho_scale_factor: u32,
    ) -> na::DMatrix<u32> {
        let max_line_length = self.calculate_max_line_length();
        let theta_axis_size = theta_scale_factor * 180;
        let rho_axis_size = (max_line_length.ceil() as u32) * rho_scale_factor;

        let mut accumulator = na::DMatrix::zeros(theta_axis_size as usize, rho_axis_size as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                let index = ((y * self.width + x) * 4) as usize;
                let gray_value = self.pixels[index];
                if gray_value >= 1 {
                    continue;
                }

                let inverted_y = self.height - y - 1;
                let coords = (x, inverted_y);

                for theta in 0..theta_axis_size {
                    let rho = self.calculate_rho(theta, theta_axis_size, coords);
                    let rho_scaled = self.scale_rho(rho, rho_axis_size, max_line_length);
                    accumulator[(theta as usize, rho_scaled as usize)] += 1;
                }
            }
        }

        accumulator
    }

    fn calculate_rho(&self, theta: u32, theta_axis_size: u32, (x, y): (u32, u32)) -> f64 {
        let angle_rad = (theta as f64) * (PI / theta_axis_size as f64);
        x as f64 * angle_rad.cos() + y as f64 * angle_rad.sin()
    }

    fn scale_rho(&self, rho: f64, rho_axis_size: u32, max_line_length: f64) -> u32 {
        let rho_axis_half = (rho_axis_size as f64) / 2.0;
        ((rho * rho_axis_half / max_line_length).round() + rho_axis_half) as u32
    }

    fn calculate_max_line_length(&self) -> f64 {
        (self.width as f64).hypot(self.height as f64)
    }

    pub fn visualize_lines(
        &self,
        accumulator: &na::DMatrix<u32>,
        theta_scale: u32,
        threshold: u32,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let max_line_length = self.calculate_max_line_length();
        let theta_axis_size = theta_scale * 180;
        let rho_axis_size = (max_line_length.ceil() as u32) * 1; // Мультипликатор ρ предполагается равным 1.
        let rho_axis_half = rho_axis_size as f64 / 2.0;

        let mut output_img = ImageBuffer::from_fn(self.width, self.height, |x, y| {
            let index = ((y * self.width + x) * 4) as usize;
            let gray = self.pixels[index];
            Rgb([gray, gray, gray])
        });

        for theta in 0..theta_axis_size {
            for rho_scaled in 0..rho_axis_size {
                let votes = accumulator[(theta as usize, rho_scaled as usize)];
                if votes < threshold {
                    continue;
                }

                let rho = (rho_scaled as f64 - rho_axis_half) * max_line_length / rho_axis_half;
                let (x1, y1, x2, y2) = self.line_from_rho_theta(theta, theta_scale, rho);

                draw_line_segment_mut(
                    &mut output_img,
                    (x1 as f32, self.height as f32 - 1.0 - y1 as f32),
                    (x2 as f32, self.height as f32 - 1.0 - y2 as f32),
                    Rgb([255, 0, 0]),
                );
            }
        }

        output_img
    }

    fn line_from_rho_theta(&self, theta: u32, theta_scale: u32, rho: f64) -> (i32, i32, i32, i32) {
        let theta_deg = (theta as f64) / theta_scale as f64;
        let angle_rad = theta_deg.to_radians();
        let (sin_theta, cos_theta) = angle_rad.sin_cos();

        let x0 = cos_theta * rho;
        let y0 = sin_theta * rho;

        let length = (self.width as f64).hypot(self.height as f64);
        let x1 = (x0 + length * (-sin_theta)) as i32;
        let y1 = (y0 + length * cos_theta) as i32;
        let x2 = (x0 - length * (-sin_theta)) as i32;
        let y2 = (y0 - length * cos_theta) as i32;

        (x1, y1, x2, y2)
    }
}
