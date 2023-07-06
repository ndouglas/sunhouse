use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vector;

/// An enum for objects that can be intersected.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Object {
  Sphere(Sphere),
}

impl Object {
  /// Compute the intersections between the object and the given ray.
  pub fn intersect(self, ray: Ray) -> Vec<Intersection> {
    match self {
      Object::Sphere(sphere) => sphere.intersect(ray),
    }
  }

  /// Create a sphere.
  pub fn sphere() -> Self {
    Object::Sphere(Sphere::default())
  }

  /// Apply a transformation to the object.
  pub fn with_transform(self, transform: Matrix) -> Self {
    match self {
      Object::Sphere(sphere) => Object::Sphere(sphere.with_transform(transform)),
    }
  }

  /// Calculate the normal vector at the given point on the object.
  pub fn normal_at(self, point: Point) -> Vector {
    match self {
      Object::Sphere(sphere) => sphere.normal_at(point),
    }
  }

  /// Get the material of the object.
  pub fn material(self) -> crate::material::Material {
    match self {
      Object::Sphere(sphere) => sphere.material,
    }
  }
}

impl Default for Object {
  fn default() -> Self {
    Object::Sphere(Sphere::default())
  }
}
