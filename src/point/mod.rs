use crate::vector::Vector;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

/// A three-dimensional point.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Point(pub f64, pub f64, pub f64);

impl Point {
  pub fn abs(self) -> Self {
    Point(self.0.abs(), self.1.abs(), self.2.abs())
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, _other: Self) -> Self {
    panic!("Cannot add two points.");
  }
}

impl Add<Vector> for Point {
  type Output = Point;

  fn add(self, other: Vector) -> Point {
    Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
  }
}

impl Neg for Point {
  type Output = Self;

  fn neg(self) -> Self {
    Point(-self.0, -self.1, -self.2)
  }
}

impl Sub for Point {
  type Output = Vector;

  fn sub(self, other: Self) -> Vector {
    Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
  }
}

impl Sub<Vector> for Point {
  type Output = Self;

  fn sub(self, other: Vector) -> Self {
    Point(self.0 - other.0, self.1 - other.1, self.2 - other.2)
  }
}

impl Div<f64> for Point {
  type Output = Self;

  fn div(self, other: f64) -> Self {
    Point(self.0 / other, self.1 / other, self.2 / other)
  }
}

impl Mul<f64> for Point {
  type Output = Self;

  fn mul(self, other: f64) -> Self {
    Point(self.0 * other, self.1 * other, self.2 * other)
  }
}
