use crate::point::Point;
use crate::tuple::Tuple;
use crate::vector::Vector;
use std::ops::Mul;

/// A 2x2 Matrix.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Matrix2x2(pub [[f64; 2]; 2]);

impl Matrix2x2 {
  pub fn identity() -> Self {
    Matrix2x2([[1.0, 0.0], [0.0, 1.0]])
  }

  pub fn transpose(&self) -> Self {
    let mut result = Matrix2x2::default();
    for i in 0..2 {
      for j in 0..2 {
        result.0[i][j] = self.0[j][i];
      }
    }
    result
  }

  pub fn determinant(&self) -> f64 {
    self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]
  }

  pub fn minor(&self, _row: usize, _col: usize) -> f64 {
    self.determinant()
  }

  pub fn cofactor(&self, row: usize, col: usize) -> f64 {
    let minor = self.minor(row, col);
    if (row + col) % 2 == 0 {
      minor
    } else {
      -minor
    }
  }

  pub fn is_invertible(&self) -> bool {
    self.determinant() != 0.0
  }

  pub fn inverse(&self) -> Self {
    let mut result = Matrix2x2::default();
    let determinant = self.determinant();
    for i in 0..2 {
      for j in 0..2 {
        let cofactor = self.cofactor(i, j);
        result.0[j][i] = cofactor / determinant;
      }
    }
    result
  }
}

impl Mul for Matrix2x2 {
  type Output = Self;

  fn mul(self, other: Matrix2x2) -> Self {
    let mut result = Matrix2x2::default();
    for i in 0..2 {
      for j in 0..2 {
        for k in 0..2 {
          result.0[i][j] += self.0[i][k] * other.0[k][j];
        }
      }
    }
    result
  }
}

impl Mul<Tuple> for Matrix2x2 {
  type Output = Tuple;

  fn mul(self, rhs: Tuple) -> Tuple {
    match rhs {
      Tuple::Point(Point(x, y, z)) => {
        let new_x = self.0[0][0] * x + self.0[0][1] * y;
        let new_y = self.0[1][0] * x + self.0[1][1] * y;
        let new_z = z;
        Tuple::new(new_x, new_y, new_z, 1.0)
      },
      Tuple::Vector(Vector(x, y, z)) => {
        let new_x = self.0[0][0] * x + self.0[0][1] * y;
        let new_y = self.0[1][0] * x + self.0[1][1] * y;
        let new_z = z;
        Tuple::new(new_x, new_y, new_z, 0.0)
      },
      Tuple::None => panic!("Cannot multiply a 2x2 matrix by a None tuple"),
    }
  }
}
