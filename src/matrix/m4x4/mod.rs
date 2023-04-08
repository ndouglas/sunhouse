use super::Matrix3x3;
use crate::point::Point;
use crate::tuple::Tuple;
use crate::vector::Vector;
use std::ops::Mul;

/// A 4x4 Matrix.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Matrix4x4(pub [[f64; 4]; 4]);

impl Matrix4x4 {
  pub fn identity() -> Self {
    Matrix4x4([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn transpose(&self) -> Self {
    let mut result = Matrix4x4::default();
    for i in 0..4 {
      for j in 0..4 {
        result.0[i][j] = self.0[j][i];
      }
    }
    result
  }

  pub fn determinant(&self) -> f64 {
    let mut result = 0.0;
    for i in 0..4 {
      result += self.0[0][i] * self.cofactor(0, i);
    }
    result
  }

  pub fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
    let mut result = Matrix3x3::default();
    let mut i = 0;
    let mut j = 0;
    for r in 0..4 {
      for c in 0..4 {
        if r != row && c != col {
          result.0[i][j] = self.0[r][c];
          j += 1;
          if j == 3 {
            j = 0;
            i += 1;
          }
        }
      }
    }
    result
  }

  pub fn minor(&self, row: usize, col: usize) -> f64 {
    self.submatrix(row, col).determinant()
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
    let mut result = Matrix4x4::default();
    let det = self.determinant();
    for i in 0..4 {
      for j in 0..4 {
        let c = self.cofactor(i, j);
        result.0[j][i] = c / det;
      }
    }
    result
  }
}

impl Mul for Matrix4x4 {
  type Output = Self;

  fn mul(self, other: Matrix4x4) -> Self {
    let mut result = Matrix4x4::default();
    for i in 0..4 {
      for j in 0..4 {
        for k in 0..4 {
          result.0[i][j] += self.0[i][k] * other.0[k][j];
        }
      }
    }
    result
  }
}

impl Mul<Tuple> for Matrix4x4 {
  type Output = Tuple;

  fn mul(self, rhs: Tuple) -> Tuple {
    match rhs {
      Tuple::Point(Point(x, y, z)) => {
        let new_x = self.0[0][0] * x + self.0[0][1] * y + self.0[0][2] * z + self.0[0][3];
        let new_y = self.0[1][0] * x + self.0[1][1] * y + self.0[1][2] * z + self.0[1][3];
        let new_z = self.0[2][0] * x + self.0[2][1] * y + self.0[2][2] * z + self.0[2][3];
        Tuple::new(new_x, new_y, new_z, 1.0)
      },
      Tuple::Vector(Vector(x, y, z)) => {
        let new_x = self.0[0][0] * x + self.0[0][1] * y + self.0[0][2] * z;
        let new_y = self.0[1][0] * x + self.0[1][1] * y + self.0[1][2] * z;
        let new_z = self.0[2][0] * x + self.0[2][1] * y + self.0[2][2] * z;
        Tuple::new(new_x, new_y, new_z, 0.0)
      },
      Tuple::None => panic!("Cannot multiply a 4x4 matrix by a None tuple"),
    }
  }
}
