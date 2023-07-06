use crate::color::Color;
use png::{BitDepth, ColorType, Encoder};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

/// A Canvas of pixels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Canvas {
  pub width: usize,
  pub height: usize,
  pub pixels: Vec<Color>,
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    Canvas {
      width,
      height,
      pixels: vec![Color::default(); width * height],
    }
  }

  pub fn set_color_at(&mut self, x: usize, y: usize, color: Color) {
    self.pixels[y * self.width + x] = color;
  }

  pub fn get_color_at(&self, x: usize, y: usize) -> Color {
    self.pixels[y * self.width + x]
  }

  /// Convert to a PPM file.
  pub fn to_ppm(&self) -> String {
    let mut ppm = "P3\n".to_string();
    ppm.push_str(&format!("{} {}\n255", self.width, self.height));
    for y in 0..self.height {
      ppm.push('\n');
      for x in 0..self.width {
        let color = self.get_color_at(x, y);
        ppm.push_str(&format!(
          "{} {} {}",
          (color.0 * 256.0) as u8,
          (color.1 * 256.0) as u8,
          (color.2 * 256.0) as u8
        ));
        if x < self.width - 1 {
          ppm.push(' ');
        }
      }
    }
    ppm.push('\n');
    ppm
  }

  /// Write the PPM to a file.
  pub fn save_ppm(&self, path: &Path) {
    let ppm = self.to_ppm();
    let display = path.display();
    let mut file = match File::create(path) {
      Err(why) => panic!("couldn't create {}: {}", display, why),
      Ok(file) => file,
    };
    match file.write_all(ppm.as_bytes()) {
      Err(why) => panic!("couldn't write to {}: {}", display, why),
      Ok(_) => println!("successfully wrote to {}", display),
    }
  }

  /// Convert to a PNG file.
  pub fn to_png(&self, path: &Path) {
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let data = self
      .pixels
      .iter()
      .flat_map(|color| {
        let r = (color.0 * 256.0) as u8;
        let g = (color.1 * 256.0) as u8;
        let b = (color.2 * 256.0) as u8;
        vec![r, g, b]
      })
      .collect::<Vec<u8>>();
    writer.write_image_data(&data).unwrap(); // Save
  }
}
