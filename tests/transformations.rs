#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;
use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use sunhouse::matrix::Matrix;
use sunhouse::point::Point;
use sunhouse::vector::Vector;

// `MatrixWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct MatrixTransWorld {
  transform: Matrix,
  inv: Matrix,
  a: Matrix,
  b: Matrix,
  c: Matrix,
  p: Point,
  p2: Point,
  p3: Point,
  p4: Point,
  t: Matrix,
  v: Vector,
  from: Point,
  to: Point,
  up: Vector,
  half_quarter: Matrix,
  full_quarter: Matrix,
}

#[given(regex = r#"^transform ← translation\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn transform_translation(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  world.transform = Matrix::translation(x, y, z);
}

#[given(regex = r#"^p ← point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn p_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  world.p = Point(x, y, z);
}

#[then(regex = r#"^transform \* p = point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn transform_p_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  let result = world.transform * world.p;
  assert_approx_eq!(result.0, x);
  assert_approx_eq!(result.1, y);
  assert_approx_eq!(result.2, z);
}

#[given(regex = r#"^inv ← inverse\(transform\)$"#)]
fn inv_inverse_transform(world: &mut MatrixTransWorld) {
  world.inv = world.transform.inverse();
}

#[then(regex = r#"^inv \* p = point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn inv_p_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  let result = world.inv * world.p;
  assert_approx_eq!(result.0, x);
  assert_approx_eq!(result.1, y);
  assert_approx_eq!(result.2, z);
}

#[given(regex = r#"^v ← vector\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn v_vector(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  world.v = Vector(x, y, z);
}

#[then(regex = r#"^transform \* v = vector\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn transform_v_vector(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  let result = world.transform * world.v;
  assert_approx_eq!(result.0, x);
  assert_approx_eq!(result.1, y);
  assert_approx_eq!(result.2, z);
}

#[then(regex = r#"^transform \* v = v$"#)]
fn transform_v_v(world: &mut MatrixTransWorld) {
  let result = world.transform * world.v;
  assert_eq!(result, world.v);
}

#[given(regex = r#"^transform ← scaling\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn transform_scaling(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  world.transform = Matrix::scaling(x, y, z);
}

#[then(regex = r#"^inv \* v = vector\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn inv_v_vector(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  let result = world.inv * world.v;
  assert_approx_eq!(result.0, x);
  assert_approx_eq!(result.1, y);
  assert_approx_eq!(result.2, z);
}

#[given(regex = r#"^half_quarter ← rotation_x\(π / 4\)$"#)]
fn half_quarter_rotation_x(world: &mut MatrixTransWorld) {
  world.half_quarter = Matrix::rotation_x(std::f64::consts::PI / 4.0);
}

#[given(regex = r#"^full_quarter ← rotation_x\(π / 2\)$"#)]
fn full_quarter_rotation_x(world: &mut MatrixTransWorld) {
  world.full_quarter = Matrix::rotation_x(std::f64::consts::PI / 2.0);
}

#[then(regex = r#"^half_quarter \* p = point\((?P<x>-?\d+), √2/2, √2/2\)$"#)]
fn half_quarter_p_point(world: &mut MatrixTransWorld, x: f64) {
  let result = world.half_quarter * world.p;
  assert_approx_eq!(result.0, x);
  assert_approx_eq!(result.1, (2.0_f64).sqrt() / 2.0);
  assert_approx_eq!(result.2, (2.0_f64).sqrt() / 2.0);
}

#[then(regex = r#"^full_quarter \* p = point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn full_quarter_p_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  let result = world.full_quarter * world.p;
  assert_approx_eq!(result.0, x);
  assert_approx_eq!(result.1, y.sqrt());
  assert_approx_eq!(result.2, z.sqrt());
}

#[given(regex = r#"^inv ← inverse\(half_quarter\)$"#)]
fn inv_inverse_half_quarter(world: &mut MatrixTransWorld) {
  world.inv = world.half_quarter.inverse();
}

#[then(regex = r#"^inv \* p = point\((?P<x>-?\d+), √2/2, -√2/2\)$"#)]
fn inv_p_point_half_quarter(world: &mut MatrixTransWorld, x: f64) {
  let result = world.inv * world.p;
  assert_approx_eq!(result.0, x);
  assert_approx_eq!(result.1, (2.0_f64).sqrt() / 2.0);
  assert_approx_eq!(result.2, -(2.0_f64).sqrt() / 2.0);
}

#[given(regex = r#"^half_quarter ← rotation_y\(π / 4\)$"#)]
fn half_quarter_rotation_y(world: &mut MatrixTransWorld) {
  world.half_quarter = Matrix::rotation_y(std::f64::consts::PI / 4.0);
}

#[then(regex = r#"^half_quarter \* p = point\(-√2/2, (?P<y>-?\d+), √2/2\)$"#)]
fn half_quarter_p_point_y(world: &mut MatrixTransWorld, y: f64) {
  let result = world.half_quarter * world.p;
  assert_approx_eq!(result.0, -(2.0_f64).sqrt() / 2.0);
  assert_approx_eq!(result.1, y);
  assert_approx_eq!(result.2, (2.0_f64).sqrt() / 2.0);
}

#[given(regex = r#"^full_quarter ← rotation_y\(π / 2\)$"#)]
fn full_quarter_rotation_y(world: &mut MatrixTransWorld) {
  world.full_quarter = Matrix::rotation_y(std::f64::consts::PI / 2.0);
}

#[given(regex = r#"^half_quarter ← rotation_z\(π / 4\)$"#)]
fn half_quarter_rotation_z(world: &mut MatrixTransWorld) {
  world.half_quarter = Matrix::rotation_z(std::f64::consts::PI / 4.0);
}

#[then(regex = r#"^half_quarter \* p = point\(-√2/2, -√2/2, (?P<z>-?\d+)\)$"#)]
fn half_quarter_p_point_z(world: &mut MatrixTransWorld, z: f64) {
  let result = world.half_quarter * world.p;
  assert_approx_eq!(result.0, -(2.0_f64).sqrt() / 2.0);
  assert_approx_eq!(result.1, -(2.0_f64).sqrt() / 2.0);
  assert_approx_eq!(result.2, z);
}

#[given(regex = r#"^full_quarter ← rotation_z\(π / 2\)$"#)]
fn full_quarter_rotation_z(world: &mut MatrixTransWorld) {
  world.full_quarter = Matrix::rotation_z(std::f64::consts::PI / 2.0);
}

#[then(regex = r#"^half_quarter \* p = point\(√2/2, 0, √2/2\)$"#)]
fn half_quarter_p_point2(world: &mut MatrixTransWorld) {
  let result = world.half_quarter * world.p;
  assert_approx_eq!(result.0, (2.0_f64).sqrt() / 2.0);
  assert_approx_eq!(result.1, 0.0);
  assert_approx_eq!(result.2, (2.0_f64).sqrt() / 2.0);
}

#[then(regex = r#"^half_quarter \* p = point\(-√2/2, √2/2, 0\)$"#)]
fn half_quarter_p_point3(world: &mut MatrixTransWorld) {
  let result = world.half_quarter * world.p;
  assert_approx_eq!(result.0, -(2.0_f64).sqrt() / 2.0);
  assert_approx_eq!(result.1, (2.0_f64).sqrt() / 2.0);
  assert_approx_eq!(result.2, 0.0);
}

#[given(
  regex = r#"^transform ← shearing\((?P<x_y>-?\d+), (?P<x_z>-?\d+), (?P<y_x>-?\d+), (?P<y_z>-?\d+), (?P<z_x>-?\d+), (?P<z_y>-?\d+)\)$"#
)]
fn transform_shearing(world: &mut MatrixTransWorld, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) {
  world.transform = Matrix::shearing(x_y, x_z, y_x, y_z, z_x, z_y);
}

#[given(regex = r#"^A ← rotation_x\(π / 2\)$"#)]
fn a_rotation_x(world: &mut MatrixTransWorld) {
  world.a = Matrix::rotation_x(std::f64::consts::PI / 2.0);
}

#[given(regex = r#"^B ← scaling\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn b_scaling(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  world.b = Matrix::scaling(x, y, z);
}

#[given(regex = r#"^C ← translation\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn c_translation(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  world.c = Matrix::translation(x, y, z);
}

#[when(regex = r#"^p2 ← A \* p$"#)]
fn p2_a_p(world: &mut MatrixTransWorld) {
  world.p2 = world.a * world.p;
}

#[then(regex = r#"^p2 = point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn p2_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.p2.0, x);
  assert_approx_eq!(world.p2.1, y);
  assert_approx_eq!(world.p2.2, z);
}

#[when(regex = r#"^p3 ← B \* p2$"#)]
fn p3_b_p2(world: &mut MatrixTransWorld) {
  world.p3 = world.b * world.p2;
}

#[then(regex = r#"^p3 = point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn p3_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.p3.0, x);
  assert_approx_eq!(world.p3.1, y);
  assert_approx_eq!(world.p3.2, z);
}

#[when(regex = r#"^p4 ← C \* p3$"#)]
fn p4_c_p3(world: &mut MatrixTransWorld) {
  world.p4 = world.c * world.p3;
}

#[then(regex = r#"^p4 = point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn p4_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.p4.0, x);
  assert_approx_eq!(world.p4.1, y);
  assert_approx_eq!(world.p4.2, z);
}

#[when(regex = r#"^T ← C \* B \* A$"#)]
fn t_c_b_a(world: &mut MatrixTransWorld) {
  world.t = world.c * world.b * world.a;
}

#[then(regex = r#"^T \* p = point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn t_p_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  let result = world.t * world.p;
  assert_approx_eq!(result.0, x);
  assert_approx_eq!(result.1, y);
  assert_approx_eq!(result.2, z);
}

#[given(regex = r#"^from ← point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn from_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  world.from = Point(x, y, z);
}

#[given(regex = r#"^to ← point\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn to_point(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  world.to = Point(x, y, z);
}

#[given(regex = r#"^up ← vector\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn up_vector(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  world.up = Vector(x, y, z);
}

#[when(regex = r#"^t ← view_transform\(from, to, up\)$"#)]
fn t_view_transform(world: &mut MatrixTransWorld) {
  world.t = Matrix::view_transform(world.from, world.to, world.up);
}

#[then(regex = r#"^t = identity_matrix$"#)]
fn t_identity_matrix(world: &mut MatrixTransWorld) {
  assert_eq!(world.t, Matrix::identity());
}

#[then(regex = r#"^t = scaling\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn t_scaling(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  assert_eq!(world.t, Matrix::scaling(x, y, z));
}

#[then(regex = r#"^t = translation\((?P<x>-?\d+), (?P<y>-?\d+), (?P<z>-?\d+)\)$"#)]
fn t_translation(world: &mut MatrixTransWorld, x: f64, y: f64, z: f64) {
  assert_eq!(world.t, Matrix::translation(x, y, z));
}

#[then(regex = r#"^t is the following (?P<rows>\d+)x(?P<cols>\d+) matrix:$"#)]
fn t_matrix(world: &mut MatrixTransWorld, step: &Step, rows: usize, cols: usize) {
  let expected = Matrix::from_rows(
    step
      .table()
      .unwrap()
      .rows
      .iter()
      .map(|row| row.iter().map(|cell| cell.parse::<f64>().unwrap()).collect())
      .collect(),
  );
  for row in 0..rows {
    for col in 0..cols {
      assert_approx_eq!(world.t.get_value(row, col), expected.get_value(row, col), 1e-5);
    }
  }
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(MatrixTransWorld::run("tests/features/transformations.feature"));
}
