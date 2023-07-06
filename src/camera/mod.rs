use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;

use crate::world::World;
use std::path::Path;

/// A camera.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Camera {
  pub hsize: usize,
  pub vsize: usize,
  pub field_of_view: f64,
  pub transform: Matrix,
  pub pixel_size: f64,
  pub half_width: f64,
  pub half_height: f64,
  pub half_view: f64,
}

impl Camera {
  /// Create a new camera.
  pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
    let transform = Matrix::identity();
    let half_view = (field_of_view / 2.0).tan();
    let aspect = hsize as f64 / vsize as f64;
    let (half_width, half_height) = if aspect >= 1.0 {
      (half_view, half_view / aspect)
    } else {
      (half_view * aspect, half_view)
    };
    let pixel_size = (half_width * 2.0) / hsize as f64;
    Camera {
      hsize,
      vsize,
      field_of_view,
      transform,
      pixel_size,
      half_width,
      half_height,
      half_view,
    }
  }

  /// Compute the ray for the given pixel.
  pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
    let xoffset = (px as f64 + 0.5) * self.pixel_size;
    let yoffset = (py as f64 + 0.5) * self.pixel_size;
    let world_x = self.half_width - xoffset;
    let world_y = self.half_height - yoffset;
    let pixel = self.transform.inverse() * Point(world_x, world_y, -self.half_view);
    let origin = self.transform.inverse() * Point::default();
    let direction = (pixel - origin).normalize();
    Ray::new(origin, direction)
  }

  /// Render the world to a canvas.
  pub fn render(&self, world: &World) -> Canvas {
    let mut canvas = Canvas::new(self.hsize, self.vsize);
    for y in 0..self.vsize {
      for x in 0..self.hsize {
        let ray = self.ray_for_pixel(x, y);
        let color = world.color_at(ray);
        canvas.set_color_at(x, y, color);
      }
    }
    canvas
  }

  /// Render a PNG of the world.
  pub fn render_png(&self, world: &World, filename: &str) {
    let canvas = self.render(world);
    canvas.to_png(Path::new(filename));
  }
}
