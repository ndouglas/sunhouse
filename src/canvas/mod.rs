use crate::color::Color;
use png::{BitDepth, ColorType, Encoder, SourceChromaticities};
use std::fs::File;
use std::io::BufWriter;
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
    let mut ppm = "P3\n5 3\n255".to_string();
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

  /// Convert to a PNG file.
  pub fn to_png(&self, path: &Path) {
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);
    let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    let source_chromaticities = SourceChromaticities::new(
      (0.31270, 0.32900),
      (0.64000, 0.33000),
      (0.30000, 0.60000),
      (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    let data = self
      .pixels
      .iter()
      .flat_map(|color| {
        let r = (color.0 * 256.0) as u8;
        let g = (color.1 * 256.0) as u8;
        let b = (color.2 * 256.0) as u8;
        let a = 255;
        vec![r, g, b, a]
      })
      .collect::<Vec<u8>>();
    writer.write_image_data(&data).unwrap(); // Save
  }
}
