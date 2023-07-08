use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;
use std::cell::RefCell;
use std::rc::Rc;

/// A plane is a flat, two-dimensional surface that extends infinitely in all
/// directions.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plane {
  pub material: Material,
  pub transform: Matrix,
  pub parent: Option<Rc<RefCell<Object>>>,
}

impl Plane {
  /// Create a new plane.
  pub fn new(transform: Matrix, material: Material) -> Self {
    Plane {
      transform,
      material,
      parent: None,
    }
  }

  /// Compute the normal vector at the given point on the plane.
  pub fn normal_at(&self, _point: Point) -> Vector {
    Vector(0.0, 1.0, 0.0)
  }

  /// Compute the intersections between the ray and the plane.
  pub fn intersect(&mut self, ray: Ray) -> Vec<Intersection> {
    if ray.direction.1.abs() < 0.0001 {
      return vec![];
    }
    let t = -ray.origin.1 / ray.direction.1;
    vec![Intersection::new(t, Object::Plane(self.clone()))]
  }

  /// Apply a transformation to the plane.
  pub fn with_transform(&self, transform: Matrix) -> Self {
    Plane {
      transform: self.transform * transform,
      ..(*self).clone()
    }
  }

  /// Return a clone of the plane with a new material.
  pub fn with_material(&self, material: Material) -> Self {
    Plane {
      material,
      ..(*self).clone()
    }
  }
}
