use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;

/// An intersection is a point where two or more lines meet.
#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
  pub t: f64,
  pub object: Object,
}

impl Intersection {
  /// Create a new intersection.
  pub fn new(t: f64, object: Object) -> Self {
    Intersection { t, object }
  }

  /// Compute the point along the ray at the given distance.
  pub fn position(self, ray: Ray) -> Point {
    ray.position(self.t)
  }
}

impl Default for Intersection {
  fn default() -> Self {
    Intersection {
      t: 0.0,
      object: Object::default(),
    }
  }
}
