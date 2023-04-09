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

  /// Light a material at a point using the eye and normal vectors.
  pub fn light(self, material: Material, point: Point, eye: Vector, normal: Vector) -> Color {
    let color = material.color * self.intensity;
    let light_vector = (self.position - point).normalize();
    let ambient = color * material.ambient;
    let light_dot_normal = light_vector.dot(normal);
    let diffuse = if light_dot_normal < 0.0 {
      Color::default()
    } else {
      color * material.diffuse * light_dot_normal
    };
    let reflect_vector = (-light_vector).reflect(normal);
    let reflect_dot_eye = reflect_vector.dot(eye);
    let specular = if reflect_dot_eye <= 0.0 {
      Color::default()
    } else {
      self.intensity * material.specular * reflect_dot_eye.powf(material.shininess)
    };
    ambient + diffuse + specular
  }
}
