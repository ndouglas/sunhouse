use crate::color::Color;

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
}
