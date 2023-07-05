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
  pub inside: bool,
}

impl Comps {
  /// Create a new `Comps` structure.
  pub fn new(t: f64, object: Object, point: Point, eyev: Vector, normalv: Vector, inside: bool) -> Self {
    Comps {
      t,
      object,
      point,
      eyev,
      normalv,
      inside,
    }
  }

  /// Prepare the `Comps` for a given ray and intersection.
  pub fn prepare(intersection: Intersection, ray: Ray) -> Self {
    let point = ray.position(intersection.t);
    let eyev = -ray.direction;
    let mut normalv = intersection.object.normal_at(point);
    let inside = normalv.dot(eyev) < 0.0;
    normalv = if inside { -normalv } else { normalv };
    Comps::new(intersection.t, intersection.object, point, eyev, normalv, inside)
  }
}
