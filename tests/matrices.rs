#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;
use cucumber::gherkin::Step;
use cucumber::{given, then, World};
use sunhouse::matrix::Matrix;
use sunhouse::matrix::Matrix2x2;
use sunhouse::matrix::Matrix3x3;
use sunhouse::matrix::Matrix4x4;
use sunhouse::point::Point;
use sunhouse::tuple::Tuple;
use sunhouse::vector::Vector;

// `MatrixWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct MatrixWorld {
  matrix_m: Matrix,
  matrix_a: Matrix,
  matrix_b: Matrix,
  matrix_c: Matrix,
  tuple_a: Tuple,
  tuple_b: Tuple,
}

#[given(regex = r"the following ?(2x2|3x3|4x4|) matrix (M|A|B):")]
fn set_matrix(world: &mut MatrixWorld, step: &Step, dimensions: String, matrix_id: String) {
  let mut m = match dimensions.as_str() {
    "2x2" => Matrix::Matrix2x2(Matrix2x2::default()),
    "3x3" => Matrix::Matrix3x3(Matrix3x3::default()),
    "4x4" => Matrix::Matrix4x4(Matrix4x4::default()),
    "" => Matrix::Matrix4x4(Matrix4x4::default()),
    _ => panic!("Unknown dimensions: {}", dimensions),
  };
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        m.set_value(row, col, value.trim().parse::<f64>().unwrap());
        col += 1;
      }
      row += 1;
    }
  }
  match matrix_id.as_str() {
    "M" => world.matrix_m = m,
    "A" => world.matrix_a = m,
    "B" => world.matrix_b = m,
    _ => panic!("Unknown matrix id: {}", matrix_id),
  }
}

#[then(regex = r"([A-Z])\[(\d+),(\d+)\] = (-?\d+(\.\d+)?)$")]
fn check_matrix_value(world: &mut MatrixWorld, id: String, row: usize, col: usize, value: f64) {
  let m = match id.as_str() {
    "M" => &world.matrix_m,
    "A" => &world.matrix_a,
    "B" => &world.matrix_b,
    _ => panic!("Unknown matrix id: {}", id),
  };
  assert_approx_eq!(m.get_value(row, col), value);
}

#[then(regex = r"([A-Z])\[(\d+),(\d+)\] = (-?\d+)/(-?\d+)$")]
fn check_matrix_value2(world: &mut MatrixWorld, id: String, row: usize, col: usize, value1: f64, value2: f64) {
  let m = match id.as_str() {
    "M" => &world.matrix_m,
    "A" => &world.matrix_a,
    "B" => &world.matrix_b,
    _ => panic!("Unknown matrix id: {}", id),
  };
  assert_approx_eq!(m.get_value(row, col), value1 / value2);
}

#[then(regex = r"([A-Z]) = ([A-Z])")]
fn check_matrix_equality(world: &mut MatrixWorld, lhs_id: String, rhs_id: String) {
  let lhs = match lhs_id.as_str() {
    "M" => &world.matrix_m,
    "A" => &world.matrix_a,
    "B" => &world.matrix_b,
    _ => panic!("Unknown matrix id: {}", lhs_id),
  };
  let rhs = match rhs_id.as_str() {
    "M" => &world.matrix_m,
    "A" => &world.matrix_a,
    "B" => &world.matrix_b,
    _ => panic!("Unknown matrix id: {}", rhs_id),
  };
  assert_eq!(lhs, rhs);
}

#[then(regex = r"([A-Z]) != ([A-Z])")]
fn check_matrix_inequality(world: &mut MatrixWorld, lhs_id: String, rhs_id: String) {
  let lhs = match lhs_id.as_str() {
    "M" => &world.matrix_m,
    "A" => &world.matrix_a,
    "B" => &world.matrix_b,
    _ => panic!("Unknown matrix id: {}", lhs_id),
  };
  let rhs = match rhs_id.as_str() {
    "M" => &world.matrix_m,
    "A" => &world.matrix_a,
    "B" => &world.matrix_b,
    _ => panic!("Unknown matrix id: {}", rhs_id),
  };
  assert_ne!(lhs, rhs);
}

#[then(regex = r"A \* B is the following ?(2x2|3x3|4x4|) matrix:")]
fn multiply_matrices(world: &mut MatrixWorld, step: &Step, _dimensions: String) {
  let m = world.matrix_a * world.matrix_b;
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        assert_approx_eq!(m.get_value(row, col), value.trim().parse::<f64>().unwrap());
        col += 1;
      }
      row += 1;
    }
  }
}

#[given(regex = r"^(a|a1|a2|b) ← tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn set_a(world: &mut MatrixWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
  match name.as_str() {
    "a" => world.tuple_a = Tuple::new(x, y, z, w),
    "b" => world.tuple_b = Tuple::new(x, y, z, w),
    _ => unreachable!("This should not happen!"),
  }
}

#[then(regex = r"^A \* b = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn multiply_matrix_by_tuple(world: &mut MatrixWorld, ex: f64, ey: f64, ez: f64, _w: f64) {
  let t = world.matrix_a * world.tuple_b;
  match t {
    Tuple::Point(Point(x, y, z)) => {
      assert_approx_eq!(x, ex);
      assert_approx_eq!(y, ey);
      assert_approx_eq!(z, ez);
    },
    Tuple::Vector(Vector(x, y, z)) => {
      assert_approx_eq!(x, ex);
      assert_approx_eq!(y, ey);
      assert_approx_eq!(z, ez);
    },
    _ => unreachable!("This should not happen!"),
  }
}

#[then(regex = r"^A \* identity_matrix = A$")]
fn multiply_matrix_by_identity_matrix(world: &mut MatrixWorld) {
  assert_eq!(
    world.matrix_a * Matrix::Matrix4x4(Matrix4x4::identity()),
    world.matrix_a
  );
}

#[then(regex = r"^identity_matrix \* a = a$")]
fn multiply_identity_matrix_by_tuple(world: &mut MatrixWorld) {
  assert_eq!(Matrix::Matrix4x4(Matrix4x4::identity()) * world.tuple_a, world.tuple_a);
}

#[then(regex = r"^transpose\(identity_matrix\) = identity_matrix$")]
fn transpose_identity_matrix(_world: &mut MatrixWorld) {
  assert_eq!(
    Matrix::Matrix4x4(Matrix4x4::identity()).transpose(),
    Matrix::Matrix4x4(Matrix4x4::identity())
  );
}

#[then(regex = r"^transpose\(transpose\(M\)\) = M$")]
fn transpose_transpose_matrix(world: &mut MatrixWorld) {
  assert_eq!(world.matrix_m.transpose().transpose(), world.matrix_m);
}

#[then(regex = r"^determinant\(identity_matrix\) = 1$")]
fn determinant_identity_matrix(_world: &mut MatrixWorld) {
  assert_eq!(Matrix::Matrix4x4(Matrix4x4::identity()).determinant(), 1.0);
}

#[then(regex = r"^submatrix\(M, ?(\d+), ?(\d+)\) is the following ?(2x2|3x3|4x4|) matrix:")]
fn submatrix(world: &mut MatrixWorld, step: &Step, x: usize, y: usize, _dimensions: String) {
  let m = world.matrix_m.submatrix(x, y);
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        assert_approx_eq!(m.get_value(row, col), value.trim().parse::<f64>().unwrap());
        col += 1;
      }
      row += 1;
    }
  }
}

#[then(regex = r"^minor\(A, ?(\d+), ?(\d+)\) = (-?\d+.?\d*)$")]
fn minor(world: &mut MatrixWorld, x: usize, y: usize, ex: f64) {
  assert_approx_eq!(world.matrix_a.minor(x, y), ex);
}

#[then(regex = r"^cofactor\(A, ?(\d+), ?(\d+)\) = (-?\d+.?\d*)$")]
fn cofactor(world: &mut MatrixWorld, x: usize, y: usize, ex: f64) {
  assert_approx_eq!(world.matrix_a.cofactor(x, y), ex);
}

#[then(regex = r"^determinant\(M\) = (-?\d+.?\d*)$")]
fn determinant(world: &mut MatrixWorld, ex: f64) {
  assert_approx_eq!(world.matrix_m.determinant(), ex);
}

#[then(regex = r"^determinant\(submatrix\(M, 1, 0\)\) = (-?\d+.?\d*)$")]
fn determinant_submatrix(world: &mut MatrixWorld, ex: f64) {
  assert_approx_eq!(world.matrix_m.submatrix(1, 0).determinant(), ex);
}

#[then(regex = r"^M is invertible$")]
fn matrix_is_invertible(world: &mut MatrixWorld) {
  assert!(world.matrix_m.is_invertible());
}

#[then(regex = r"^M is not invertible$")]
fn matrix_is_not_invertible(world: &mut MatrixWorld) {
  assert!(!world.matrix_m.is_invertible());
}

#[then(regex = r"^inverse\(M\) is the following ?(2x2|3x3|4x4|) matrix:")]
fn inverse(world: &mut MatrixWorld, step: &Step, _dimensions: String) {
  let m = world.matrix_m.inverse();
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        assert_approx_eq!(m.get_value(row, col), value.trim().parse::<f64>().unwrap());
        col += 1;
      }
      row += 1;
    }
  }
}

#[then(regex = r"^inverse\(identity_matrix\) = identity_matrix$")]
fn inverse_identity_matrix(_world: &mut MatrixWorld) {
  assert_eq!(
    Matrix::Matrix4x4(Matrix4x4::identity()).inverse(),
    Matrix::Matrix4x4(Matrix4x4::identity())
  );
}

#[then(regex = r"^A \* inverse\(A\) = identity_matrix$")]
fn multiply_matrix_by_inverse(world: &mut MatrixWorld) {
  assert_eq!(
    world.matrix_a * world.matrix_a.inverse(),
    Matrix::Matrix4x4(Matrix4x4::identity())
  );
}

#[then(regex = r"^inverse\(A\) \* A = identity_matrix$")]
fn multiply_inverse_by_matrix(world: &mut MatrixWorld) {
  assert_eq!(
    world.matrix_a.inverse() * world.matrix_a,
    Matrix::Matrix4x4(Matrix4x4::identity())
  );
}

#[then(regex = r"^inverse\(M\) is the following ?(2x2|3x3|4x4|) matrix:")]
fn inverse2(world: &mut MatrixWorld, step: &Step, _dimensions: String) {
  let m = world.matrix_m.inverse();
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        assert_approx_eq!(m.get_value(row, col), value.trim().parse::<f64>().unwrap());
        col += 1;
      }
      row += 1;
    }
  }
}

#[then(regex = r"^inverse\(M\) is the following ?(2x2|3x3|4x4|) matrix:")]
fn inverse3(world: &mut MatrixWorld, step: &Step, _dimensions: String) {
  let m = world.matrix_m.inverse();
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        assert_approx_eq!(m.get_value(row, col), value.trim().parse::<f64>().unwrap());
        col += 1;
      }
      row += 1;
    }
  }
}

#[then(regex = r"^transpose\(A\) is the following ?(2x2|3x3|4x4|) matrix:")]
fn transpose(world: &mut MatrixWorld, step: &Step, _dimensions: String) {
  let m = world.matrix_a.transpose();
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        assert_approx_eq!(m.get_value(row, col), value.trim().parse::<f64>().unwrap());
        col += 1;
      }
      row += 1;
    }
  }
}

#[given(regex = r"^A ← transpose\(identity_matrix\)$")]
fn transpose_identity_matrix2(world: &mut MatrixWorld) {
  world.matrix_a = Matrix::Matrix4x4(Matrix4x4::identity()).transpose();
}

#[given(regex = r"^B ← inverse\(A\)$")]
fn b_is_inverse_a(world: &mut MatrixWorld) {
  world.matrix_b = world.matrix_a.inverse();
}

#[given(regex = r"^B ← submatrix\(A, ?(\d+), ?(\d+)\)$")]
fn b_is_submatrix_a(world: &mut MatrixWorld, x: usize, y: usize) {
  world.matrix_b = world.matrix_a.submatrix(x, y);
}

#[then(regex = r"^A = identity_matrix$")]
fn identity_matrix2(world: &mut MatrixWorld) {
  assert_eq!(world.matrix_a, Matrix::Matrix4x4(Matrix4x4::identity()));
}

#[then(regex = r"^determinant\((A|B|M)\) = (-?\d+.?\d*)$")]
fn determinant2(world: &mut MatrixWorld, name: String, ex: f64) {
  match name.as_str() {
    "A" => assert_approx_eq!(world.matrix_a.determinant(), ex),
    "B" => assert_approx_eq!(world.matrix_b.determinant(), ex),
    "M" => assert_approx_eq!(world.matrix_m.determinant(), ex),
    _ => unreachable!("This should not happen!"),
  }
}

#[then(regex = r"^submatrix\(A, ?(\d), ?(\d)\) is the following ?(2x2|3x3|4x4|) matrix:")]
fn submatrix2(world: &mut MatrixWorld, step: &Step, x: usize, y: usize, _dimensions: String) {
  let m = world.matrix_a.submatrix(x, y);
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        assert_approx_eq!(m.get_value(row, col), value.trim().parse::<f64>().unwrap());
        col += 1;
      }
      row += 1;
    }
  }
}

#[then(regex = r"^inverse\(A\) is the following ?(2x2|3x3|4x4|) matrix:")]
fn inverse_is_matrix(world: &mut MatrixWorld, step: &Step, _dimensions: String) {
  let m = world.matrix_a.inverse();
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        assert_approx_eq!(m.get_value(row, col), value.trim().parse::<f64>().unwrap(), 0.0001);
        col += 1;
      }
      row += 1;
    }
  }
}

#[then(regex = r"^(A|B|M) is invertible$")]
fn is_invertible(world: &mut MatrixWorld, name: String) {
  match name.as_str() {
    "A" => assert!(world.matrix_a.is_invertible()),
    "B" => assert!(world.matrix_b.is_invertible()),
    "M" => assert!(world.matrix_m.is_invertible()),
    _ => unreachable!("This should not happen!"),
  }
}

#[then(regex = r"^(A|B|M) is not invertible$")]
fn is_not_invertible(world: &mut MatrixWorld, name: String) {
  match name.as_str() {
    "A" => assert!(!world.matrix_a.is_invertible()),
    "B" => assert!(!world.matrix_b.is_invertible()),
    "M" => assert!(!world.matrix_m.is_invertible()),
    _ => unreachable!("This should not happen!"),
  }
}

#[then(regex = r"^(A|B|M) is the following ?(2x2|3x3|4x4|) matrix:")]
fn is_matrix(world: &mut MatrixWorld, step: &Step, name: String, _dimensions: String) {
  let m = match name.as_str() {
    "A" => &world.matrix_a,
    "B" => &world.matrix_b,
    "M" => &world.matrix_m,
    _ => unreachable!("This should not happen!"),
  };
  let mut row = 0;
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let mut col = 0;
      for value in line.iter() {
        assert_approx_eq!(m.get_value(row, col), value.trim().parse::<f64>().unwrap(), 1e-5);
        col += 1;
      }
      row += 1;
    }
  }
}

#[given(regex = r"^C ← A \* B$")]
fn c_is_a_times_b(world: &mut MatrixWorld) {
  world.matrix_c = world.matrix_a * world.matrix_b;
}

#[then(regex = r"^C \* inverse\(B\) = A$")]
fn c_times_inverse_b_is_a(world: &mut MatrixWorld) {
  let lhs = world.matrix_c * world.matrix_b.inverse();
  for i in 0..4 {
    for j in 0..4 {
      assert_approx_eq!(lhs.get_value(i, j), world.matrix_a.get_value(i, j), 1e-5);
    }
  }
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(MatrixWorld::run("tests/features/matrices.feature"));
}
