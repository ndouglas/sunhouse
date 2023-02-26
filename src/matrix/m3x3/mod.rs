use std::ops::Mul;

/// A 3x3 Matrix.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Matrix3x3(pub [[f64; 3]; 3]);

impl Mul for Matrix3x3 {
  type Output = Self;

  fn mul(self, other: Matrix3x3) -> Self {
    let mut result = Matrix3x3::default();
    for i in 0..3 {
      for j in 0..3 {
        for k in 0..3 {
          result.0[i][j] += self.0[i][k] * other.0[k][j];
        }
      }
    }
    result
  }
}
