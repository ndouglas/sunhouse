use std::ops::Mul;

/// A 2x2 Matrix.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Matrix2x2(pub [[f64; 2]; 2]);

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
