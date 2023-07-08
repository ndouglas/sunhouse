use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::plane::Plane;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::test_shape::TestShape;
use crate::vector::Vector;
use std::cell::RefCell;
use std::rc::Rc;

/// An enum for objects that can be intersected.
#[derive(Clone, Debug, PartialEq)]
pub enum Object {
  Plane(Plane),
  Sphere(Sphere),
  TestShape(TestShape),
}

impl Object {
  /// Compute the intersections between the object and the given ray.
  pub fn intersect(&mut self, ray: Ray) -> Vec<Intersection> {
    match self {
      Object::Plane(ref mut plane) => plane.intersect(ray),
      Object::Sphere(ref mut sphere) => sphere.intersect(ray),
      Object::TestShape(ref mut test_shape) => test_shape.intersect(ray),
    }
  }

  /// Create a sphere.
  pub fn sphere() -> Self {
    Object::Sphere(Sphere::default())
  }

  /// Create a test shape.
  pub fn test_shape() -> Self {
    Object::TestShape(TestShape::default())
  }

  /// Create a plane.
  pub fn plane() -> Self {
    Object::Plane(Plane::default())
  }

  /// Apply a transformation to the object.
  pub fn with_transform(&self, transform: Matrix) -> Self {
    match self {
      Object::Plane(plane) => Object::Plane(plane.with_transform(transform)),
      Object::Sphere(sphere) => Object::Sphere(sphere.with_transform(transform)),
      Object::TestShape(test_shape) => Object::TestShape(test_shape.with_transform(transform)),
    }
  }

  /// Retrieve the transform of the object.
  pub fn transform(&self) -> Matrix {
    match self {
      Object::Plane(plane) => plane.transform,
      Object::Sphere(sphere) => sphere.transform,
      Object::TestShape(test_shape) => test_shape.transform,
    }
  }

  /// Calculate the normal vector at the given point on the object.
  pub fn normal_at(&self, point: Point) -> Vector {
    match self {
      Object::Plane(plane) => plane.normal_at(point),
      Object::Sphere(sphere) => sphere.normal_at(point),
      Object::TestShape(test_shape) => test_shape.normal_at(point),
    }
  }

  /// Get the material of the object.
  pub fn material(&self) -> Material {
    match self {
      Object::Plane(plane) => plane.material,
      Object::Sphere(sphere) => sphere.material,
      Object::TestShape(test_shape) => test_shape.material,
    }
  }

  /// Return a clone of the object with a new material.
  pub fn with_material(&self, material: Material) -> Self {
    match self {
      Object::Plane(plane) => Object::Plane(plane.with_material(material)),
      Object::Sphere(sphere) => Object::Sphere(sphere.with_material(material)),
      Object::TestShape(test_shape) => Object::TestShape(test_shape.with_material(material)),
    }
  }

  /// Provide access to the parent of the object, if any.
  pub fn parent(&self) -> Option<Rc<RefCell<Object>>> {
    match self {
      Object::Plane(plane) => plane.parent.clone(),
      Object::Sphere(sphere) => sphere.parent.clone(),
      Object::TestShape(test_shape) => test_shape.parent.clone(),
    }
  }
}

impl Default for Object {
  fn default() -> Self {
    Object::Sphere(Sphere::default())
  }
}
