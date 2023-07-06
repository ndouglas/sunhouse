#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;
use cucumber::{given, then, when, World};
use sunhouse::camera::Camera;
use sunhouse::canvas::Canvas;
use sunhouse::matrix::Matrix;
use sunhouse::point::Point;
use sunhouse::ray::Ray;
use sunhouse::vector::Vector;
use sunhouse::world::World as RenderWorld;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  pub hsize: usize,
  pub vsize: usize,
  pub field_of_view: f64,
  pub c: Camera,
  pub r: Ray,
  pub w: RenderWorld,
  pub from: Point,
  pub to: Point,
  pub up: Vector,
  pub image: Canvas,
}

#[given(regex = r#"^hsize ← (\d+)$"#)]
fn hsize_is(world: &mut TestWorld, hsize: usize) {
  world.hsize = hsize;
}

#[given(regex = r#"^vsize ← (\d+)$"#)]
fn vsize_is(world: &mut TestWorld, vsize: usize) {
  world.vsize = vsize;
}

#[given(regex = r#"^field_of_view ← π/2$"#)]
fn field_of_view_is(world: &mut TestWorld) {
  world.field_of_view = std::f64::consts::PI / 2.0;
}

#[when(regex = r#"^c ← camera\(hsize, vsize, field_of_view\)$"#)]
fn camera_is_props(world: &mut TestWorld) {
  world.c = Camera::new(world.hsize, world.vsize, world.field_of_view);
}

#[given(regex = r#"^c ← camera\((\d+), (\d+), π/2\)$"#)]
#[when(regex = r#"^c ← camera\((\d+), (\d+), π/2\)$"#)]
fn camera_is(world: &mut TestWorld, hsize: usize, vsize: usize) {
  world.c = Camera::new(hsize, vsize, std::f64::consts::PI / 2.0);
}

#[then(regex = r#"^c\.hsize = (\d+)$"#)]
fn camera_hsize_is(world: &mut TestWorld, hsize: usize) {
  assert_eq!(world.c.hsize, hsize);
}

#[then(regex = r#"^c\.vsize = (\d+)$"#)]
fn camera_vsize_is(world: &mut TestWorld, vsize: usize) {
  assert_eq!(world.c.vsize, vsize);
}

#[then(regex = r#"^c\.field_of_view = π/2$"#)]
fn camera_field_of_view_is(world: &mut TestWorld) {
  assert_approx_eq!(world.c.field_of_view, std::f64::consts::PI / 2.0);
}

#[then(regex = r#"^c\.transform = identity_matrix$"#)]
fn camera_transform_is(world: &mut TestWorld) {
  assert_eq!(world.c.transform, Matrix::identity());
}

#[then(regex = r#"^c\.pixel_size = (\d+\.\d+)$"#)]
fn camera_pixel_size_is(world: &mut TestWorld, pixel_size: f64) {
  assert_approx_eq!(world.c.pixel_size, pixel_size);
}

#[when(regex = r#"^r ← ray_for_pixel\(c, (\d+), (\d+)\)$"#)]
fn ray_for_pixel_is(world: &mut TestWorld, x: usize, y: usize) {
  world.r = world.c.ray_for_pixel(x, y);
}

#[then(regex = r#"^r\.origin = point\((-?\d+), (-?\d+), (-?\d+)\)$"#)]
#[then(regex = r#"^r\.origin = point\((-?\d+\.\d+), (-?\d+\.\d+), (-?\d+\.\d+)\)$"#)]
fn ray_origin_is(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.r.origin.0, x);
  assert_approx_eq!(world.r.origin.1, y);
  assert_approx_eq!(world.r.origin.2, z);
}

#[then(regex = r#"^r\.direction = vector\((-?\d+), (-?\d+), (-?\d+)\)$"#)]
#[then(regex = r#"^r\.direction = vector\((-?\d+\.\d+), (-?\d+\.\d+), (-?\d+\.\d+)\)$"#)]
fn ray_direction_is(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  assert_approx_eq!(world.r.direction.0, x, 1e-5);
  assert_approx_eq!(world.r.direction.1, y, 1e-5);
  assert_approx_eq!(world.r.direction.2, z, 1e-5);
}

#[when(regex = r#"^c\.transform ← rotation_y\(π/4\) \* translation\(0, -2, 5\)$"#)]
fn camera_transform_is_props(world: &mut TestWorld) {
  world.c.transform = Matrix::rotation_y(std::f64::consts::PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0);
}

#[then(regex = r#"^r\.direction = vector\((\d+\.\d+), √2/2, -√2/2\)$"#)]
fn ray_direction_is_props_x(world: &mut TestWorld, x: f64) {
  assert_approx_eq!(world.r.direction.0, x, 1e-5);
  assert_approx_eq!(world.r.direction.1, std::f64::consts::SQRT_2 / 2.0, 1e-5);
  assert_approx_eq!(world.r.direction.2, -std::f64::consts::SQRT_2 / 2.0, 1e-5);
}

#[then(regex = r#"^r\.direction = vector\(√2/2, (-?\d+), -√2/2\)$"#)]
#[then(regex = r#"^r\.direction = vector\(√2/2, (\d+\.\d+), -√2/2\)$"#)]
fn ray_direction_is_props_y(world: &mut TestWorld, y: f64) {
  assert_approx_eq!(world.r.direction.0, std::f64::consts::SQRT_2 / 2.0);
  assert_approx_eq!(world.r.direction.1, y, 1e-5);
  assert_approx_eq!(world.r.direction.2, -std::f64::consts::SQRT_2 / 2.0, 1e-5);
}

#[given(regex = r#"^w ← default_world\(\)$"#)]
fn world_is(world: &mut TestWorld) {
  world.w = RenderWorld::default();
}

#[given(regex = r#"^from ← point\((-?\d+), (-?\d+), (-?\d+)\)$"#)]
fn from_is(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.from = Point(x, y, z);
}

#[given(regex = r#"^to ← point\((-?\d+), (-?\d+), (-?\d+)\)$"#)]
fn to_is(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.to = Point(x, y, z);
}

#[given(regex = r#"^up ← vector\((-?\d+), (-?\d+), (-?\d+)\)$"#)]
fn up_is(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.up = Vector(x, y, z);
}

#[given(regex = r#"^c.transform ← view_transform\(from, to, up\)$"#)]
fn camera_transform_is_view(world: &mut TestWorld) {
  world.c.transform = Matrix::view_transform(world.from, world.to, world.up);
}

#[when(regex = r#"^image ← render\(c, w\)$"#)]
fn image_is(world: &mut TestWorld) {
  world.image = world.w.render(&world.c);
}

#[then(regex = r#"^pixel_at\(image, (\d+), (\d+)\) = color\((-?\d+\.\d+), (-?\d+\.\d+), (-?\d+\.\d+)\)$"#)]
fn pixel_at_is(world: &mut TestWorld, x: usize, y: usize, r: f64, g: f64, b: f64) {
  assert_approx_eq!(world.image.get_color_at(x, y).0, r, 1e-5);
  assert_approx_eq!(world.image.get_color_at(x, y).1, g, 1e-5);
  assert_approx_eq!(world.image.get_color_at(x, y).2, b, 1e-5);
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/camera.feature"));
}
