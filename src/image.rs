use crate::{color::Color, utils::clamp};

const DEFAULT_WIDTH: usize = 256;
const DEFAULT_HEIGHT: usize = 256;

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            data: vec![Color::default(); DEFAULT_WIDTH * DEFAULT_HEIGHT],
        }
    }
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Color::default(); width * height],
        }
    }
    pub fn set_pixel(&mut self, pixel_color: Color, x: usize, y: usize) {
        self.data[y * self.width + x] = pixel_color;
    }
    pub fn print_ppm(&self, samples_per_pixel: usize) {
        let scale = 1.0 / samples_per_pixel as f64;
        println!("P3\n{} {}\n255", self.width, self.height);
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let color = self.data[y * self.width + x];
                let color = color * scale;
                println!(
                    "{} {} {}",
                    (256.0 * clamp(color.0.sqrt(), 0.0, 0.999)) as u8,
                    (256.0 * clamp(color.1.sqrt(), 0.0, 0.999)) as u8,
                    (256.0 * clamp(color.2.sqrt(), 0.0, 0.999)) as u8
                );
            }
        }
    }
}
