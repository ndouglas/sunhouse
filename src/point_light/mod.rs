use crate::color::Color;
use crate::material::Material;
use crate::point::Point;
use crate::vector::Vector;

/// A light source with no size, existing at a single point in space.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct PointLight {
  pub position: Point,
  pub intensity: Color,
}

impl PointLight {
  /// Create a new point light.
  pub fn new(position: Point, intensity: Color) -> Self {
    PointLight { position, intensity }
  }

  /// Get the ambient color of the light.
  pub fn ambient_light(self, material: Material) -> Color {
    self.intensity * material.color * material.ambient
  }

  /// Get the diffuse color of the light.
  pub fn diffuse_light(self, material: Material, point: Point, normal: Vector) -> Color {
    let effective_color = material.color * self.intensity;
    let light_vector = (self.position - point).normalize();
    let light_dot_normal = light_vector.dot(normal);
    if light_dot_normal < 0.0 {
      Color::new(0.0, 0.0, 0.0)
    } else {
      effective_color * material.diffuse * light_dot_normal
    }
  }

  /// Get the specular color of the light.
  pub fn specular_light(self, material: Material, point: Point, eye: Vector, normal: Vector) -> Color {
    let effective_color = material.color * self.intensity;
    let light_vector = (self.position - point).normalize();
    let light_dot_normal = light_vector.dot(normal);
    if light_dot_normal < 0.0 {
      Color::new(0.0, 0.0, 0.0)
    } else {
      let reflect_vector = (-light_vector).reflect(normal);
      let reflect_dot_eye = reflect_vector.dot(eye);
      if reflect_dot_eye <= 0.0 {
        Color::new(0.0, 0.0, 0.0)
      } else {
        let factor = reflect_dot_eye.powf(material.shininess);
        effective_color * material.specular * factor
      }
    }
  }

  /// Light a material at a point using the eye and normal vectors.
  pub fn light(self, material: Material, point: Point, eye: Vector, normal: Vector, in_shadow: bool) -> Color {
    if in_shadow {
      self.ambient_light(material)
    } else {
      self.ambient_light(material)
        + self.diffuse_light(material, point, normal)
        + self.specular_light(material, point, eye, normal)
    }
  }
}

impl From<(Point, Color)> for PointLight {
  fn from(tuple: (Point, Color)) -> Self {
    PointLight::new(tuple.0, tuple.1)
  }
}
