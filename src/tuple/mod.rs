use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use crate::point::Point;
use crate::vector::Vector;

// Distinguish between points and vectors.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum Tuple {
  Point(Point),
  Vector(Vector),
  #[default]
  None,
}

impl Tuple {
  /// Create a new tuple.
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
    if w > 0.5 {
      Tuple::Point(Point(x, y, z))
    } else {
      Tuple::Vector(Vector(x, y, z))
    }
  }

  pub fn abs(self) -> Self {
    match self {
      Tuple::Point(point) => Tuple::Point(point.abs()),
      Tuple::Vector(vector) => Tuple::Vector(vector.abs()),
      Tuple::None => Tuple::None,
    }
  }

  pub fn is_point(self) -> bool {
    matches!(self, Tuple::Point(_))
  }

  pub fn is_vector(self) -> bool {
    matches!(self, Tuple::Vector(_))
  }

  pub fn is_none(self) -> bool {
    matches!(self, Tuple::None)
  }
}

impl From<Point> for Tuple {
  fn from(point: Point) -> Self {
    Tuple::Point(point)
  }
}

impl From<Vector> for Tuple {
  fn from(vector: Vector) -> Self {
    Tuple::Vector(vector)
  }
}

impl Add for Tuple {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    match (self, other) {
      (Tuple::Point(_), Tuple::Point(_)) => panic!("Cannot add two points."),
      (Tuple::Vector(lhs), Tuple::Vector(rhs)) => Tuple::Vector(lhs + rhs),
      (Tuple::Point(lhs), Tuple::Vector(rhs)) => Tuple::Point(lhs + rhs),
      (Tuple::Vector(lhs), Tuple::Point(rhs)) => Tuple::Point(lhs + rhs),
      _ => panic!("Operation does not make sense."),
    }
  }
}

impl Sub for Tuple {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    match (self, other) {
      (Tuple::Point(Point(x1, y1, z1)), Tuple::Point(Point(x2, y2, z2))) => {
        Tuple::Vector(Vector(x1 - x2, y1 - y2, z1 - z2))
      },
      (Tuple::Vector(Vector(x1, y1, z1)), Tuple::Vector(Vector(x2, y2, z2))) => {
        Tuple::Vector(Vector(x1 - x2, y1 - y2, z1 - z2))
      },
      (Tuple::Point(Point(x1, y1, z1)), Tuple::Vector(Vector(x2, y2, z2))) => {
        Tuple::Point(Point(x1 - x2, y1 - y2, z1 - z2))
      },
      (Tuple::Vector(Vector(x1, y1, z1)), Tuple::Point(Point(x2, y2, z2))) => {
        Tuple::Vector(Vector(x1 - x2, y1 - y2, z1 - z2))
      },
      _ => Tuple::None,
    }
  }
}

impl Neg for Tuple {
  type Output = Self;

  fn neg(self) -> Self {
    match self {
      Tuple::Point(Point(x, y, z)) => Tuple::Point(-Point(x, y, z)),
      Tuple::Vector(Vector(x, y, z)) => Tuple::Vector(-Vector(x, y, z)),
      _ => Tuple::None,
    }
  }
}

impl Mul<f64> for Tuple {
  type Output = Self;

  fn mul(self, other: f64) -> Self {
    match self {
      Tuple::Point(point) => Tuple::Point(point * other),
      Tuple::Vector(vector) => Tuple::Vector(vector * other),
      _ => Tuple::None,
    }
  }
}

impl Div<f64> for Tuple {
  type Output = Self;

  fn div(self, other: f64) -> Self {
    match self {
      Tuple::Point(point) => Tuple::Point(point / other),
      Tuple::Vector(vector) => Tuple::Vector(vector / other),
      _ => Tuple::None,
    }
  }
}
