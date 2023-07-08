use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

/// The TestShape struct represents a test shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TestShape {
  pub transform: Matrix,
  pub material: Material,
  pub saved_ray: Option<Ray>,
}

impl TestShape {
  /// Create a new unit sphere.
  pub fn unit() -> Self {
    TestShape {
      transform: Matrix::identity(),
      material: Material::default(),
      saved_ray: None,
    }
  }

  /// Compute the intersections between the ray and the sphere.
  pub fn intersect(&mut self, ray: Ray) -> Vec<Intersection> {
    self.saved_ray = Some(ray);
    vec![]
  }

  /// Compute the normal vector at the given point on the test shape.
  pub fn normal_at(self, point: Point) -> Vector {
    Vector(point.0, point.1, point.2)
  }

  /// Apply a transformation to the test shape.
  pub fn with_transform(self, transform: Matrix) -> Self {
    TestShape {
      transform: self.transform * transform,
      ..self
    }
  }
}

impl Default for TestShape {
  fn default() -> Self {
    TestShape {
      transform: Matrix::identity(),
      material: Material::default(),
      saved_ray: None,
    }
  }
}
