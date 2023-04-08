use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

/// A sphere.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub transform: Matrix,
}

impl Sphere {
  /// Create a new unit sphere.
  pub fn unit() -> Self {
    Sphere {
      center: Point::default(),
      radius: 1.0,
      transform: Matrix::identity(),
    }
  }

  /// Create a new sphere.
  pub fn new(center: Point, radius: f64, transform: Matrix) -> Self {
    Sphere {
      center,
      radius,
      transform,
    }
  }

  /// Compute the normal vector at the given point on the sphere.
  pub fn normal_at(self, point: Point) -> Vector {
    (point - self.center).normalize()
  }

  /// Compute the intersections between the ray and the sphere.
  pub fn intersect(self, ray: Ray) -> Vec<Intersection> {
    let ray2 = ray.transform(self.transform.inverse());
    let sphere_to_ray = ray2.origin - self.center;
    let a = ray2.direction.dot(ray2.direction);
    let b = 2.0 * ray2.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - self.radius * self.radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
      vec![]
    } else {
      let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
      let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
      vec![
        Intersection::new(t1, Object::Sphere(self)),
        Intersection::new(t2, Object::Sphere(self)),
      ]
    }
  }
}

impl Default for Sphere {
  fn default() -> Self {
    Sphere::unit()
  }
}
