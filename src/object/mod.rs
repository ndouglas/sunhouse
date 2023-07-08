use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::test_shape::TestShape;
use crate::vector::Vector;

/// An enum for objects that can be intersected.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Object {
  Sphere(Sphere),
  TestShape(TestShape),
}

impl Object {
  /// Compute the intersections between the object and the given ray.
  pub fn intersect(&mut self, ray: Ray) -> Vec<Intersection> {
    match self {
      Object::Sphere(ref mut sphere) => sphere.intersect(ray),
      Object::TestShape(ref mut test_shape) => test_shape.intersect(ray),
    }
  }

  /// Create a sphere.
  pub fn sphere() -> Self {
    Object::Sphere(Sphere::default())
  }

  /// Create a test shape.
  pub fn test_shape() -> Self {
    Object::TestShape(TestShape::default())
  }

  /// Apply a transformation to the object.
  pub fn with_transform(self, transform: Matrix) -> Self {
    match self {
      Object::Sphere(sphere) => Object::Sphere(sphere.with_transform(transform)),
      Object::TestShape(test_shape) => Object::TestShape(test_shape.with_transform(transform)),
    }
  }

  /// Retrieve the transform of the object.
  pub fn transform(self) -> Matrix {
    match self {
      Object::Sphere(sphere) => sphere.transform,
      Object::TestShape(test_shape) => test_shape.transform,
    }
  }

  /// Calculate the normal vector at the given point on the object.
  pub fn normal_at(self, point: Point) -> Vector {
    match self {
      Object::Sphere(sphere) => sphere.normal_at(point),
      Object::TestShape(test_shape) => test_shape.normal_at(point),
    }
  }

  /// Get the material of the object.
  pub fn material(self) -> crate::material::Material {
    match self {
      Object::Sphere(sphere) => sphere.material,
      Object::TestShape(test_shape) => test_shape.material,
    }
  }
}

impl Default for Object {
  fn default() -> Self {
    Object::Sphere(Sphere::default())
  }
}
