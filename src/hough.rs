use bytes::Bytes;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;
use nalgebra::DMatrix;
use std::f64::consts::PI;

use crate::image_processing::HandleRgbaComponents;

impl HandleRgbaComponents {
    pub fn hough_transform(
        &self,
        theta_steps: u32,
        rho_steps: u32,
        edge_threshold: u8,
    ) -> (DMatrix<u32>, Vec<((f32, f32), (f32, f32))>) {
        let (width, height) = (self.width as f64, self.height as f64);
        let max_rho = (width.powi(2) + height.powi(2)).sqrt();
        let mut accumulator = DMatrix::zeros(theta_steps as usize, rho_steps as usize);

        // Collect edge points
        let edges: Vec<(f64, f64)> = self
            .pixels
            .chunks_exact(4)
            .enumerate()
            .filter_map(|(i, chunk)| {
                let x = (i % self.width as usize) as f64;
                let y = (i / self.width as usize) as f64;
                (chunk[0] < edge_threshold).then_some((x, y))
            })
            .collect();

        // Accumulate votes
        for &(x, y) in &edges {
            for theta in 0..theta_steps {
                let angle = (theta as f64 * PI) / theta_steps as f64;
                let rho = x * angle.cos() + y * angle.sin();
                let rho_idx =
                    ((rho + max_rho) * rho_steps as f64 / (2.0 * max_rho)).round() as usize;

                if rho_idx < rho_steps as usize {
                    accumulator[(theta as usize, rho_idx)] += 1;
                }
            }
        }

        let segments = self.detect_lines(&accumulator, theta_steps, rho_steps, max_rho);
        (accumulator, segments)
    }

    fn detect_lines(
        &self,
        accumulator: &DMatrix<u32>,
        theta_steps: u32,
        rho_steps: u32,
        max_rho: f64,
    ) -> Vec<((f32, f32), (f32, f32))> {
        let threshold = accumulator.max() * 3 / 4;
        let mut segments = Vec::new();

        for theta in 0..accumulator.nrows() {
            for rho in 0..accumulator.ncols() {
                if accumulator[(theta, rho)] < threshold {
                    continue;
                }

                let angle = (theta as f64 * PI) / theta_steps as f64;
                let rho_value = (rho as f64 * 2.0 * max_rho / rho_steps as f64) - max_rho;

                if let Some(segment) = self.calculate_line_segment(angle, rho_value) {
                    segments.push(segment);
                }
            }
        }

        segments
    }

    fn calculate_line_segment(&self, theta: f64, rho: f64) -> Option<((f32, f32), (f32, f32))> {
        let (w, h) = (self.width as f64, self.height as f64);
        let (cos_theta, sin_theta) = (theta.cos(), theta.sin());
        let mut points = Vec::with_capacity(2);

        // Calculate intersections with image boundaries
        for x in [0.0, w] {
            let y = (rho - x * cos_theta) / sin_theta;
            if (0.0..=h).contains(&y) {
                points.push((x, y));
            }
        }

        for y in [0.0, h] {
            let x = (rho - y * sin_theta) / cos_theta;
            if (0.0..=w).contains(&x) {
                points.push((x, y));
            }
        }

        points.dedup();
        if points.len() >= 2 {
            let (x1, y1) = (points[0].0 as f32, points[0].1 as f32);
            let (x2, y2) = (points[1].0 as f32, points[1].1 as f32);
            Some(((x1, y1), (x2, y2)))
        } else {
            None
        }
    }

    pub fn draw_segments(&self, segments: &[((f32, f32), (f32, f32))], color: [u8; 4]) -> Self {
        let mut img = RgbaImage::from_raw(self.width, self.height, self.pixels.to_vec()).unwrap();

        for &((x1, y1), (x2, y2)) in segments {
            draw_line_segment_mut(
                &mut img,
                (x1, self.height as f32 - y1),
                (x2, self.height as f32 - y2),
                Rgba(color),
            );
        }

        Self {
            width: self.width,
            height: self.height,
            pixels: Bytes::from(img.into_raw()),
        }
    }
}
