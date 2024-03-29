use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;
use std::cell::RefCell;
use std::rc::Rc;

/// A sphere.
#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
  pub center: Point,
  pub radius: f64,
  pub transform: Matrix,
  pub material: Material,
  pub parent: Option<Rc<RefCell<Object>>>,
}

impl Sphere {
  /// Create a new unit sphere.
  pub fn unit() -> Self {
    Sphere {
      center: Point::default(),
      radius: 1.0,
      transform: Matrix::identity(),
      material: Material::default(),
      parent: None,
    }
  }

  /// Create a new sphere.
  pub fn new(center: Point, radius: f64, transform: Matrix, material: Material) -> Self {
    Sphere {
      center,
      radius,
      transform,
      material,
      parent: None,
    }
  }

  /// Create a new glass sphere.
  pub fn glass() -> Self {
    Sphere {
      center: Point::default(),
      radius: 1.0,
      transform: Matrix::identity(),
      material: Material::glass(),
      parent: None,
    }
  }

  /// Compute the normal vector at the given point on the sphere.
  pub fn normal_at(&self, point: Point) -> Vector {
    // (point - self.center).normalize()
    let object_point = self.transform.inverse() * point;
    let object_normal = object_point - Point::default();
    let world_normal = self.transform.inverse().transpose() * object_normal;
    world_normal.normalize()
  }

  /// Compute the intersections between the ray and the sphere.
  pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
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
        Intersection::new(t1, Object::Sphere(self.clone())),
        Intersection::new(t2, Object::Sphere(self.clone())),
      ]
    }
  }

  /// Apply a transformation to the sphere.
  pub fn with_transform(&self, transform: Matrix) -> Self {
    Sphere {
      transform: self.transform * transform,
      ..(*self).clone()
    }
  }

  /// Return a clone of the sphere with a new material.
  pub fn with_material(&self, material: Material) -> Self {
    Sphere {
      material,
      ..(*self).clone()
    }
  }
}

impl Default for Sphere {
  fn default() -> Self {
    Sphere::unit()
  }
}
