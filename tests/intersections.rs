#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;

use cucumber::{given, then, when, World};

use sunhouse::comps::Comps;
use sunhouse::intersection::Intersection;
use sunhouse::object::Object;
use sunhouse::point::Point;

use sunhouse::ray::Ray;
use sunhouse::sphere::Sphere;

use sunhouse::vector::Vector;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  pub s: Sphere,
  pub i: Intersection,
  pub r: Ray,
  pub shape: Object,
  pub comps: Comps,
}

#[given(regex = r#"^s ← sphere\(\)$"#)]
fn sphere_is(world: &mut TestWorld) {
  world.s = Sphere::unit();
}

#[when(regex = r#"^i ← intersection\((\d+\.?\d*), s\)$"#)]
fn intersection_is(world: &mut TestWorld, t: f64) {
  world.i = Intersection::new(t, Object::Sphere(world.s));
}

#[then(regex = r#"^i\.t = (\d+\.?\d*)$"#)]
fn intersection_t_is(world: &mut TestWorld, t: f64) {
  assert_approx_eq!(world.i.t, t);
}

#[then(regex = r#"^i\.object = s$"#)]
fn intersection_object_is(world: &mut TestWorld) {
  assert_eq!(world.i.object, Object::Sphere(world.s));
}

#[given(
  regex = r#"^r ← ray\(point\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\), vector\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)\)$"#
)]
fn ray_is(world: &mut TestWorld, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) {
  world.r = Ray::new(Point(x, y, z), Vector(dx, dy, dz));
}

#[given(regex = r#"^shape ← sphere\(\)$"#)]
fn shape_is(world: &mut TestWorld) {
  world.shape = Object::Sphere(Sphere::unit());
}

#[given(regex = r#"^i ← intersection\((\d+\.?\d*), shape\)$"#)]
fn intersection_is_shape(world: &mut TestWorld, t: f64) {
  world.i = Intersection::new(t, world.shape);
}

#[when(regex = r#"^comps ← prepare_computations\(i, r\)$"#)]
fn prepare_computations(world: &mut TestWorld) {
  world.comps = Comps::prepare(world.i, world.r);
}

#[then(regex = r#"^comps\.t = i\.t$"#)]
fn comps_t_is(world: &mut TestWorld) {
  assert_approx_eq!(world.comps.t, world.i.t);
}

#[then(regex = r#"^comps\.object = i\.object$"#)]
fn comps_object_is(world: &mut TestWorld) {
  assert_eq!(world.comps.object, world.i.object);
}

#[then(regex = r#"^comps\.point = point\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)$"#)]
fn comps_point_is(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.comps.point.0, x);
  assert_approx_eq!(world.comps.point.1, y);
  assert_approx_eq!(world.comps.point.2, z);
}

#[then(regex = r#"^comps\.eyev = vector\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)$"#)]
fn comps_eyev_is(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.comps.eyev.0, x);
  assert_approx_eq!(world.comps.eyev.1, y);
  assert_approx_eq!(world.comps.eyev.2, z);
}

#[then(regex = r#"^comps\.normalv = vector\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)$"#)]
fn comps_normalv_is(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.comps.normalv.0, x);
  assert_approx_eq!(world.comps.normalv.1, y);
  assert_approx_eq!(world.comps.normalv.2, z);
}

#[then(regex = r#"^comps\.inside = (true|false)$"#)]
fn comps_inside_is(world: &mut TestWorld, inside: bool) {
  assert_eq!(world.comps.inside, inside);
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/intersections.feature"));
}
