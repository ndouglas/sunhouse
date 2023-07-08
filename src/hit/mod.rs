use crate::intersection::Intersection;

/// Implement `hit()` for a vector of intersections.
pub trait Hit {
  /// Return the intersection with the lowest non-negative t value.
  fn hit(&self) -> Option<Intersection>;
}

impl Hit for Vec<Intersection> {
  fn hit(&self) -> Option<Intersection> {
    self
      .iter()
      .filter(|i| i.t >= 0.0)
      .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
      .cloned()
  }
}
