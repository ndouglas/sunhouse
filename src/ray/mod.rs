use crate::matrix::Matrix;
use crate::point::Point;
use crate::vector::Vector;

/// A ray.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Ray {
  pub origin: Point,
  pub direction: Vector,
}

impl Ray {
  /// Create a new ray.
  pub fn new(origin: Point, direction: Vector) -> Self {
    Ray { origin, direction }
  }

  /// Compute the point along the ray at the given distance.
  pub fn position(self, distance: f64) -> Point {
    self.origin + self.direction * distance
  }

  /// Transform the ray by the given matrix.
  pub fn transform(self, matrix: Matrix) -> Self {
    Ray {
      origin: matrix * self.origin,
      direction: matrix * self.direction,
    }
  }
}
