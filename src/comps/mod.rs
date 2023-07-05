use crate::intersection::Intersection;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

/// A data structure encapsulating some precomputed information relating to the
/// intersection.
#[derive(Debug, Default, Clone)]
pub struct Comps {
  pub t: f64,
  pub object: Object,
  pub point: Point,
  pub eyev: Vector,
  pub normalv: Vector,
}

impl Comps {
  /// Create a new `Comps` structure.
  pub fn new(t: f64, object: Object, point: Point, eyev: Vector, normalv: Vector) -> Self {
    Comps {
      t,
      object,
      point,
      eyev,
      normalv,
    }
  }

  /// Prepare the `Comps` for a given ray and intersection.
  pub fn prepare(intersection: Intersection, ray: Ray) -> Self {
    let point = ray.position(intersection.t);
    let eyev = -ray.direction;
    let normalv = intersection.object.normal_at(point);
    Comps::new(intersection.t, intersection.object, point, eyev, normalv)
  }
}
