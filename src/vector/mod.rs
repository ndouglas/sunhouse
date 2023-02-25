use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use crate::point::Point;

/// A three-dimensional vector.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector(pub f64, pub f64, pub f64);

impl Vector {
  pub fn abs(self) -> Self {
    Vector(self.0.abs(), self.1.abs(), self.2.abs())
  }
}

impl Add for Vector {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
  }
}

impl Add<Point> for Vector {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
  }
}

impl Neg for Vector {
  type Output = Self;

  fn neg(self) -> Self {
    Vector(-self.0, -self.1, -self.2)
  }
}

impl Sub for Vector {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
  }
}

impl Sub<Point> for Vector {
  type Output = Point;

  fn sub(self, other: Point) -> Point {
    Point(self.0 - other.0, self.1 - other.1, self.2 - other.2)
  }
}

impl Mul<f64> for Vector {
  type Output = Self;

  fn mul(self, other: f64) -> Self {
    Vector(self.0 * other, self.1 * other, self.2 * other)
  }
}

impl Div<f64> for Vector {
  type Output = Self;

  fn div(self, other: f64) -> Self {
    Vector(self.0 / other, self.1 / other, self.2 / other)
  }
}
