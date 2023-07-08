use assert_approx_eq::assert_approx_eq;
use cucumber::{given, then, when, World};
use std::f64::consts::PI;
use sunhouse::intersection::Intersection;
use sunhouse::material::Material;
use sunhouse::matrix::Matrix;
use sunhouse::object::Object;
use sunhouse::point::Point;
use sunhouse::ray::Ray;
use sunhouse::test_shape::TestShape;
use sunhouse::vector::Vector;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  pub s: Object,
  pub m: Material,
  pub n: Vector,
  pub r: Ray,
  pub xs: Vec<Intersection>,
  pub m2: Matrix,
}

#[given(regex = r#"^s ← test_shape\(\)$"#)]
fn test_shape_is(world: &mut TestWorld) {
  world.s = Object::TestShape(TestShape::default());
}

#[then(regex = r#"^s\.transform = identity_matrix$"#)]
fn test_shape_transform_is_identity_matrix(world: &mut TestWorld) {
  assert_eq!(world.s.transform(), Matrix::identity());
}

#[when(regex = r#"^set_transform\(s, translation\((.*), (.*), (.*)\)\)$"#)]
fn set_transform(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.s = world.s.with_transform(Matrix::translation(x, y, z));
}

#[then(regex = r#"^s\.transform = translation\((.*), (.*), (.*)\)$"#)]
fn test_shape_transform_is_translation(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_eq!(world.s.transform(), Matrix::translation(x, y, z));
}

#[when(regex = r#"^m ← s\.material$"#)]
fn m_is_material(world: &mut TestWorld) {
  world.m = world.s.material();
}

#[then(regex = r#"^m = material\(\)$"#)]
fn m_is_default_material(world: &mut TestWorld) {
  assert_eq!(world.m, Material::default());
}

#[given(regex = r#"^m ← material\(\)$"#)]
fn m_is_material2(world: &mut TestWorld) {
  world.m = Material::default();
}

#[given(regex = r#"^m\.ambient ← (.*)$"#)]
fn m_ambient_is(world: &mut TestWorld, ambient: f64) {
  world.m.ambient = ambient;
}

#[when(regex = r#"^s\.material ← m$"#)]
fn s_material_is_m(world: &mut TestWorld) {
  world.s = world.s.with_material(world.m);
}

#[then(regex = r#"^s\.material = m$"#)]
fn s_material_is_m2(world: &mut TestWorld) {
  assert_eq!(world.s.material(), world.m);
}

#[given(regex = r#"^r ← ray\(point\((.*), (.*), (.*)\), vector\((.*), (.*), (.*)\)\)$"#)]
fn r_is_ray(world: &mut TestWorld, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) {
  world.r = Ray::new(Point(x1, y1, z1), Vector(x2, y2, z2));
}

#[when(regex = r#"^set_transform\(s, scaling\((.*), (.*), (.*)\)\)$"#)]
fn set_transform_scaling(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.s = world.s.with_transform(Matrix::scaling(x, y, z));
}

#[when(regex = r#"^xs ← intersect\(s, r\)$"#)]
fn xs_is_intersect(world: &mut TestWorld) {
  world.xs = world.s.intersect(world.r);
}

#[then(regex = r#"^s\.saved_ray\.origin = point\((.*), (.*), (.*)\)$"#)]
fn s_saved_ray_origin_is_point(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  if let Object::TestShape(ref mut test_shape) = world.s {
    assert_eq!(test_shape.saved_ray.unwrap().origin, Point(x, y, z));
  } else {
    panic!("Expected TestShape");
  }
}

#[then(regex = r#"^s\.saved_ray\.direction = vector\((.*), (.*), (.*)\)$"#)]
fn s_saved_ray_direction_is_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  if let Object::TestShape(ref mut test_shape) = world.s {
    assert_eq!(test_shape.saved_ray.unwrap().direction, Vector(x, y, z));
  } else {
    panic!("Expected TestShape");
  }
}

#[when(regex = r#"^n ← normal_at\(s, point\((.*), ([^√]*), (.*)\)\)$"#)]
fn n_is_normal_at(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.n = world.s.normal_at(Point(x, y, z));
}

#[then(regex = r#"^n = vector\((.*), (.*), (.*)\)$"#)]
fn n_is_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.n.0, x, 1e-5);
  assert_approx_eq!(world.n.1, y, 1e-5);
  assert_approx_eq!(world.n.2, z, 1e-5);
}

#[given(regex = r#"^m ← scaling\((.*), (.*), (.*)\) \* rotation_z\(π/5\)$"#)]
fn m_is_scaling_rotation_z(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.m2 = Matrix::scaling(x, y, z) * Matrix::rotation_z(PI / 5.0);
}

#[when(regex = r#"^set_transform\(s, m\)$"#)]
fn set_transform_m(world: &mut TestWorld) {
  world.s = world.s.with_transform(world.m2);
}

#[when(regex = r#"^n ← normal_at\(s, point\(0, √2/2, -√2/2\)\)$"#)]
fn n_is_normal_at_point(world: &mut TestWorld) {
  world.n = world
    .s
    .normal_at(Point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
}

#[then(regex = r#"^s\.parent is nothing$"#)]
fn s_parent_is_nothing(world: &mut TestWorld) {
  assert_eq!(world.s.parent(), None);
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/shapes.feature"));
}
