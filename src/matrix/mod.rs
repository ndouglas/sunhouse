use std::ops::Mul;

pub mod m2x2;
pub use m2x2::Matrix2x2;
pub mod m3x3;
pub use m3x3::Matrix3x3;
pub mod m4x4;
pub use m4x4::Matrix4x4;

/// A Matrix Enum.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum Matrix {
  /// A 2x2 Matrix.
  Matrix2x2(Matrix2x2),
  /// A 3x3 Matrix.
  Matrix3x3(Matrix3x3),
  /// A 4x4 Matrix.
  Matrix4x4(Matrix4x4),
  /// Default.
  #[default]
  None,
}

impl Matrix {
  pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
    match self {
      Matrix::Matrix2x2(m) => m.0[row][col] = value,
      Matrix::Matrix3x3(m) => m.0[row][col] = value,
      Matrix::Matrix4x4(m) => m.0[row][col] = value,
      Matrix::None => panic!("Matrix is None!"),
    }
  }

  pub fn get_value(&self, row: usize, col: usize) -> f64 {
    match self {
      Matrix::Matrix2x2(m) => m.0[row][col],
      Matrix::Matrix3x3(m) => m.0[row][col],
      Matrix::Matrix4x4(m) => m.0[row][col],
      Matrix::None => panic!("Matrix is None!"),
    }
  }
}

impl Mul for Matrix {
  type Output = Matrix;

  fn mul(self, rhs: Matrix) -> Matrix {
    match (self, rhs) {
      (Matrix::Matrix2x2(m1), Matrix::Matrix2x2(m2)) => Matrix::Matrix2x2(m1 * m2),
      (Matrix::Matrix3x3(m1), Matrix::Matrix3x3(m2)) => Matrix::Matrix3x3(m1 * m2),
      (Matrix::Matrix4x4(m1), Matrix::Matrix4x4(m2)) => Matrix::Matrix4x4(m1 * m2),
      _ => panic!("Matrix dimensions do not match!"),
    }
  }
}
