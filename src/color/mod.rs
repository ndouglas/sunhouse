use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

/// A three-dimensional color.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
  pub fn abs(self) -> Self {
    Color(self.0.abs(), self.1.abs(), self.2.abs())
  }
}

impl Add for Color {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
  }
}

impl Sub for Color {
  type Output = Self;

  fn sub(self, other: Self) -> Color {
    Color(self.0 - other.0, self.1 - other.1, self.2 - other.2)
  }
}

impl Div<f64> for Color {
  type Output = Self;

  fn div(self, other: f64) -> Self {
    Color(self.0 / other, self.1 / other, self.2 / other)
  }
}

impl Mul<f64> for Color {
  type Output = Self;

  fn mul(self, other: f64) -> Self {
    Color(self.0 * other, self.1 * other, self.2 * other)
  }
}

impl Mul for Color {
  type Output = Self;

  fn mul(self, other: Color) -> Self {
    Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
  }
}
