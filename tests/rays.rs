#[allow(clippy::too_many_arguments)]
use cucumber::{given, then, when, World};
use sunhouse::matrix::Matrix;
use sunhouse::point::Point;
use sunhouse::ray::Ray;
use sunhouse::vector::Vector;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  origin: Point,
  direction: Vector,
  m: Matrix,
  ray: Ray,
  r2: Ray,
}

#[given(regex = r#"^origin ← point\((.*), (.*), (.*)\)$"#)]
fn origin_is_point(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.origin = Point(x, y, z);
}

#[given(regex = r#"^direction ← vector\((.*), (.*), (.*)\)$"#)]
fn direction_is_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.direction = Vector(x, y, z);
}

#[when(regex = r#"^r ← ray\(origin, direction\)$"#)]
fn ray_is_origin_direction(world: &mut TestWorld) {
  world.ray = Ray::new(world.origin, world.direction);
}

#[then(regex = r#"^r\.origin = origin$"#)]
fn ray_origin_is_origin(world: &mut TestWorld) {
  assert_eq!(world.ray.origin, world.origin);
}

#[then(regex = r#"^r\.direction = direction$"#)]
fn ray_direction_is_direction(world: &mut TestWorld) {
  assert_eq!(world.ray.direction, world.direction);
}

#[given(regex = r#"^r ← ray\(point\((.*), (.*), (.*)\), vector\((.*), (.*), (.*)\)\)$"#)]
fn ray_is_point_vector(world: &mut TestWorld, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) {
  world.ray = Ray::new(Point(x1, y1, z1), Vector(x2, y2, z2));
}

#[then(regex = r#"^position\(r, (.*)\) = point\((.*), (.*), (.*)\)$"#)]
fn position_ray_is_point(world: &mut TestWorld, distance: f64, x: f64, y: f64, z: f64) {
  assert_eq!(world.ray.position(distance), Point(x, y, z));
}

#[given(regex = r#"^m ← translation\((.*), (.*), (.*)\)$"#)]
fn m_is_translation(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.m = Matrix::translation(x, y, z);
}

#[when(regex = r#"^r2 ← transform\(r, m\)$"#)]
fn r2_is_transform(world: &mut TestWorld) {
  world.r2 = world.ray.transform(world.m);
}

#[then(regex = r#"^r2\.origin = point\((.*), (.*), (.*)\)$"#)]
fn r2_origin_is_point(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_eq!(world.r2.origin, Point(x, y, z));
}

#[then(regex = r#"^r2\.direction = vector\((.*), (.*), (.*)\)$"#)]
fn r2_direction_is_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_eq!(world.r2.direction, Vector(x, y, z));
}

#[given(regex = r#"^m ← scaling\((.*), (.*), (.*)\)$"#)]
fn m_is_scaling(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.m = Matrix::scaling(x, y, z);
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/rays.feature"));
}
