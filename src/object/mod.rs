use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::sphere::Sphere;

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
}
