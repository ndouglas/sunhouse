use std::ops::Mul;

/// A 4x4 Matrix.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Matrix4x4(pub [[f64; 4]; 4]);

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
