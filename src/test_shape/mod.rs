use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;
use std::cell::RefCell;
use std::rc::Rc;

/// The TestShape struct represents a test shape.
#[derive(Clone, Debug, PartialEq)]
pub struct TestShape {
  pub transform: Matrix,
  pub material: Material,
  pub saved_ray: Option<Ray>,
  pub parent: Option<Rc<RefCell<Object>>>,
}

impl TestShape {
  /// Create a new unit sphere.
  pub fn unit() -> Self {
    TestShape {
      transform: Matrix::identity(),
      material: Material::default(),
      saved_ray: None,
      parent: None,
    }
  }

  /// Compute the intersections between the ray and the sphere.
  pub fn intersect(&mut self, ray: Ray) -> Vec<Intersection> {
    let ray2 = ray.transform(self.transform.inverse());
    self.saved_ray = Some(ray2);
    vec![]
  }

  /// Compute the normal vector at the given point on the test shape.
  pub fn normal_at(&self, point: Point) -> Vector {
    let object_point = self.transform.inverse() * point;
    let object_normal = object_point - Point::default();
    let world_normal = self.transform.inverse().transpose() * object_normal;
    world_normal.normalize()
  }

  /// Apply a transformation to the test shape.
  pub fn with_transform(&self, transform: Matrix) -> Self {
    TestShape {
      transform: self.transform * transform,
      ..(*self).clone()
    }
  }

  /// Return a clone of the test shape with a new material.
  pub fn with_material(&self, material: Material) -> Self {
    TestShape {
      material,
      ..(*self).clone()
    }
  }
}

impl Default for TestShape {
  fn default() -> Self {
    TestShape {
      transform: Matrix::identity(),
      material: Material::default(),
      saved_ray: None,
      parent: None,
    }
  }
}
