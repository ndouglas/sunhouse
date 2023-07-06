use crate::color::Color;
use crate::point::Point;
use crate::point_light::PointLight;
use crate::vector::Vector;

/// Encapsulates the surface color and attributes from the reflection model.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Material {
  pub color: Color,
  pub ambient: f64,
  pub diffuse: f64,
  pub specular: f64,
  pub shininess: f64,
}

impl Material {
  pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
    Material {
      color,
      ambient,
      diffuse,
      specular,
      shininess,
    }
  }

  pub fn glass() -> Self {
    Material {
      color: Color::new(1.0, 1.0, 1.0),
      ambient: 0.0,
      diffuse: 0.1,
      specular: 0.9,
      shininess: 200.0,
    }
  }

  pub fn lighting(self, light: PointLight, point: Point, eye: Vector, normal: Vector) -> Color {
    light.light(self, point, eye, normal)
  }
}

impl Default for Material {
  fn default() -> Self {
    Material {
      color: Color::new(1.0, 1.0, 1.0),
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0,
    }
  }
}
