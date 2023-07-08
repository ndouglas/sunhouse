use assert_approx_eq::assert_approx_eq;
use cucumber::{given, then, when, World};

use sunhouse::intersection::Intersection;

use sunhouse::object::Object;
use sunhouse::plane::Plane;
use sunhouse::point::Point;
use sunhouse::ray::Ray;

use sunhouse::vector::Vector;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  pub p: Plane,
  pub n1: Vector,
  pub n2: Vector,
  pub n3: Vector,
  pub r: Ray,
  pub xs: Vec<Intersection>,
}

#[given(regex = r#"^p ← plane\(\)$"#)]
fn plane_is(world: &mut TestWorld) {
  world.p = Plane::default();
}

#[when(regex = r#"^n1 ← local_normal_at\(p, point\((.*), (.*), (.*)\)\)$"#)]
fn n1_is_normal_at(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.n1 = world.p.normal_at(Point(x, y, z));
}

#[when(regex = r#"^n2 ← local_normal_at\(p, point\((.*), (.*), (.*)\)\)$"#)]
fn n2_is_normal_at(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.n2 = world.p.normal_at(Point(x, y, z));
}

#[when(regex = r#"^n3 ← local_normal_at\(p, point\((.*), (.*), (.*)\)\)$"#)]
fn n3_is_normal_at(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.n3 = world.p.normal_at(Point(x, y, z));
}

#[then(regex = r#"^n1 = vector\((.*), (.*), (.*)\)$"#)]
fn n1_is_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.n1.0, x, 1e-5);
  assert_approx_eq!(world.n1.1, y, 1e-5);
  assert_approx_eq!(world.n1.2, z, 1e-5);
}

#[then(regex = r#"^n2 = vector\((.*), (.*), (.*)\)$"#)]
fn n2_is_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.n2.0, x, 1e-5);
  assert_approx_eq!(world.n2.1, y, 1e-5);
  assert_approx_eq!(world.n2.2, z, 1e-5);
}

#[then(regex = r#"^n3 = vector\((.*), (.*), (.*)\)$"#)]
fn n3_is_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.n3.0, x, 1e-5);
  assert_approx_eq!(world.n3.1, y, 1e-5);
  assert_approx_eq!(world.n3.2, z, 1e-5);
}

#[given(regex = r#"^r ← ray\(point\((.*), (.*), (.*)\), vector\((.*), (.*), (.*)\)\)$"#)]
fn r_is_ray(world: &mut TestWorld, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) {
  world.r = Ray::new(Point(x, y, z), Vector(dx, dy, dz));
}

#[when(regex = r#"^xs ← local_intersect\(p, r\)$"#)]
fn xs_is_local_intersect(world: &mut TestWorld) {
  world.xs = world.p.intersect(world.r);
}

#[then(regex = r#"^xs is empty$"#)]
fn xs_is_empty(world: &mut TestWorld) {
  assert!(world.xs.is_empty());
}

#[then(regex = r#"^xs\.count = (\d+)$"#)]
fn xs_len_is(world: &mut TestWorld, len: usize) {
  assert_eq!(world.xs.len(), len);
}

#[then(regex = r#"^xs\[(\d+)\].t = (\d+\.?\d*)$"#)]
fn xs_t_is(world: &mut TestWorld, index: usize, t: f64) {
  assert_approx_eq!(world.xs[index].t, t);
}

#[then(regex = r#"^xs\[(\d+)\].object = p$"#)]
fn xs_object_is(world: &mut TestWorld, index: usize) {
  assert_eq!(world.xs[index].object, Object::Plane(world.p.clone()));
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/planes.feature"));
}
