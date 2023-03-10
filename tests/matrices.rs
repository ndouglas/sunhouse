#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;
use cucumber::gherkin::Step;
use cucumber::{given, then, World};
use sunhouse::matrix::Matrix;
use sunhouse::matrix::Matrix2x2;
use sunhouse::matrix::Matrix3x3;
use sunhouse::matrix::Matrix4x4;

// `MatrixWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct MatrixWorld {
  matrix_m: Matrix,
  matrix_a: Matrix,
  matrix_b: Matrix,
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

#[then(regex = r"([A-Z])\[(\d+),(\d+)\] = (-?\d+(\.\d+)?)")]
fn check_matrix_value(world: &mut MatrixWorld, id: String, row: usize, col: usize, value: f64) {
  let m = match id.as_str() {
    "M" => &world.matrix_m,
    "A" => &world.matrix_a,
    "B" => &world.matrix_b,
    _ => panic!("Unknown matrix id: {}", id),
  };
  assert_approx_eq!(m.get_value(row, col), value);
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

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(MatrixWorld::run("tests/features/matrices.feature"));
}
