#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;
use cucumber::{given, then, when, World};
use sunhouse::intersection::Intersection;
use sunhouse::matrix::Matrix;
use sunhouse::object::Object;
use sunhouse::point::Point;
use sunhouse::ray::Ray;
use sunhouse::sphere::Sphere;
use sunhouse::vector::Vector;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  ray: Ray,
  shape: Sphere,
  sphere: Sphere,
  t: Matrix,
  xs: Vec<Intersection>,
  xs1: Vec<Intersection>,
}

#[given(regex = r#"^r ← ray\(point\((.*), (.*), (.*)\), vector\((.*), (.*), (.*)\)\)$"#)]
fn ray_is_point_vector(world: &mut TestWorld, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) {
  world.ray = Ray::new(Point(x1, y1, z1), Vector(x2, y2, z2));
}

#[given(regex = r#"^s ← sphere\(\)$"#)]
fn sphere_is(world: &mut TestWorld) {
  world.sphere = Sphere::default();
}

#[when(regex = r#"^xs1 ← intersect\(s, r\)$"#)]
fn xs1_is_intersect(world: &mut TestWorld) {
  world.xs1 = world.sphere.intersect(world.ray);
}

#[then(regex = r#"^xs1\.count = (.*)$"#)]
fn xs1_count_is(world: &mut TestWorld, count: usize) {
  assert_eq!(world.xs1.len(), count);
}

#[then(regex = r#"^xs1\[(\d+)\] = (.*)$"#)]
fn xs1_index_is(world: &mut TestWorld, idx: usize, x: f64) {
  assert_approx_eq!(world.xs1[idx].t, x);
}

#[given(regex = r#"^shape ← sphere\(\)$"#)]
fn shape_is(world: &mut TestWorld) {
  world.shape = Sphere::default();
}

#[when(regex = r#"^xs ← intersect\(shape, r\)$"#)]
fn xs_is_intersect(world: &mut TestWorld) {
  world.xs = world.shape.intersect(world.ray);
}

#[then(regex = r#"^xs\.count = (.*)$"#)]
fn xs_count_is(world: &mut TestWorld, count: usize) {
  assert_eq!(world.xs.len(), count);
}

#[then(regex = r#"^xs\[(\d+)\] = (.*)$"#)]
fn xs_index_is(world: &mut TestWorld, idx: usize, x: f64) {
  assert_approx_eq!(world.xs[idx].t, x);
}

#[then(regex = r#"^xs\[(\d+)\]\.object = shape$"#)]
fn xs_index_object_is(world: &mut TestWorld, idx: usize) {
  assert_eq!(world.xs[idx].object, Object::Sphere(world.shape));
}

#[then(regex = r#"^s.transform = identity_matrix$"#)]
fn s_transform_is_identity(world: &mut TestWorld) {
  assert_eq!(world.sphere.transform, Matrix::identity());
}

#[given(regex = r#"^t ← translation\((.*), (.*), (.*)\)$"#)]
fn t_is_translation(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.t = Matrix::translation(x, y, z);
}

#[when(regex = r#"^set_transform\(s, t\)$"#)]
fn set_transform_s_t(world: &mut TestWorld) {
  world.sphere.transform = world.t;
}

#[then(regex = r#"^s.transform = t$"#)]
fn s_transform_is_t(world: &mut TestWorld) {
  assert_eq!(world.sphere.transform, world.t);
}

#[when(regex = r#"^set_transform\(s, scaling\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)\)$"#)]
fn set_transform_shape_scaling(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.sphere.transform = Matrix::scaling(x, y, z);
}

#[then(regex = r#"^xs1\[(\d+)\]\.t = (.*)$"#)]
fn xs1_index_t_is(world: &mut TestWorld, idx: usize, x: f64) {
  assert_approx_eq!(world.xs1[idx].t, x);
}

#[given(regex = r#"^set_transform\(s, translation\((.*), (.*), (.*)\)\)$"#)]
#[when(regex = r#"^set_transform\(s, translation\((.*), (.*), (.*)\)\)$"#)]
fn set_transform_shape_translation(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.sphere.transform = Matrix::translation(x, y, z);
}

#[when(regex = r#"^n ← normal_at\(s, point\(√3/3, √3/3, √3/3\)\)$"#)]
fn n_is_normal_at(world: &mut TestWorld) {
  let sqrt3 = 3.0_f64.sqrt();
  let x = sqrt3 / 3.0;
  world.shape.normal_at(Point(x, x, x));
}

#[when(regex = r#"^n ← normal_at\(s, point\(([^√]*), ([^√]*), ([^√]*)\)\)$"#)]
fn n_is_normal_at_point(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.shape.normal_at(Point(x, y, z));
}

#[then(regex = r#"^n = vector\(√3/3, √3/3, √3/3\)$"#)]
fn n_is_vector(world: &mut TestWorld) {
  let sqrt3 = 3.0_f64.sqrt();
  let x = sqrt3 / 3.0;
  assert_eq!(world.shape.normal_at(Point(x, x, x)), Vector(x, x, x));
}

#[then(regex = r#"^n = vector\(([\d\.-]*), ([\d\.-]*), ([\d\.-]*)\)$"#)]
fn n_is_vector2(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  let vector = world.shape.normal_at(Point(x, y, z));
  assert_approx_eq!(vector.0, x, 1e-5);
  assert_approx_eq!(vector.1, y, 1e-5);
  assert_approx_eq!(vector.2, z, 1e-5);
}

#[then(regex = r#"^n = normalize\(vector\((\d+), (\d+), (\d+)\)\)$"#)]
fn n_is_normalize_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_eq!(world.shape.normal_at(Point(x, y, z)), Vector(x, y, z).normalize());
}

#[then(regex = r#"^n = normalize\(n\)$"#)]
fn n_is_normalize_n(world: &mut TestWorld) {
  let sqrt3 = 3.0_f64.sqrt();
  let x = sqrt3 / 3.0;
  assert_eq!(world.shape.normal_at(Point(x, x, x)), Vector(x, x, x).normalize());
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/spheres.feature"));
}
