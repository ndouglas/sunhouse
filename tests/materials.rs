#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;
use cucumber::{given, then, when, World};
use sunhouse::color::Color;
use sunhouse::material::Material;
use sunhouse::point::Point;
use sunhouse::point_light::PointLight;
use sunhouse::vector::Vector;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  eyev: Vector,
  light: PointLight,
  m: Material,
  normalv: Vector,
  position: Point,
  result: Color,
  in_shadow: bool,
}

#[given(regex = r#"^m ← material\(\)$"#)]
fn m_is_material(world: &mut TestWorld) {
  world.m = Material::default();
}

#[given(regex = r#"^position ← point\((.*), (.*), (.*)\)$"#)]
fn position_is_point(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.position = Point(x, y, z);
}

#[then(regex = r#"^m\.color = color\((.*), (.*), (.*)\)$"#)]
fn m_color_is_color(world: &mut TestWorld, r: f64, g: f64, b: f64) {
  assert_eq!(world.m.color, Color(r, g, b));
}

#[then(regex = r#"^m.ambient = (.*)$"#)]
fn m_ambient_is(world: &mut TestWorld, a: f64) {
  assert_eq!(world.m.ambient, a);
}

#[then(regex = r#"^m.diffuse = (.*)$"#)]
fn m_diffuse_is(world: &mut TestWorld, d: f64) {
  assert_eq!(world.m.diffuse, d);
}

#[then(regex = r#"^m.specular = (.*)$"#)]
fn m_specular_is(world: &mut TestWorld, s: f64) {
  assert_eq!(world.m.specular, s);
}

#[then(regex = r#"^m.shininess = (.*)$"#)]
fn m_shininess_is(world: &mut TestWorld, s: f64) {
  assert_eq!(world.m.shininess, s);
}

#[given(regex = r#"^eyev ← vector\(([^√]*), ([^√]*), ([^√]*)\)$"#)]
fn eyev_is_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.eyev = Vector(x, y, z);
}

#[given(regex = r#"^eyev ← vector\(0, √2/2, -√2/2\)$"#)]
fn eyev_is_vector2(world: &mut TestWorld) {
  world.eyev = Vector(0.0, 0.7071067811865475, -0.7071067811865475);
}

#[given(regex = r#"^normalv ← vector\(0, 0, -1\)$"#)]
fn normalv_is_vector(world: &mut TestWorld) {
  world.normalv = Vector(0.0, 0.0, -1.0);
}

#[given(regex = r#"^normalv ← vector\(0, √2/2, √2/2\)$"#)]
fn normalv_is_vector2(world: &mut TestWorld) {
  world.normalv = Vector(0.0, 0.7071067811865475, 0.7071067811865475);
}

#[given(regex = r#"^light ← point_light\(point\((.*), (.*), (.*)\), color\((.*), (.*), (.*)\)\)$"#)]
fn light_is_point_light(world: &mut TestWorld, x: f64, y: f64, z: f64, r: f64, g: f64, b: f64) {
  world.light = PointLight::new(Point(x, y, z), Color(r, g, b));
}

#[then(regex = r#"^lighting\(m, light, position, eyev, normalv\) = color\((.*), (.*), (.*)\)$"#)]
fn lighting_is_color(world: &mut TestWorld, r: f64, g: f64, b: f64) {
  let c = world
    .light
    .light(world.m, world.position, world.eyev, world.normalv, false);
  assert_eq!(c, Color(r, g, b));
}

#[when(regex = r#"^result ← lighting\(m, light, position, eyev, normalv\)$"#)]
fn result_is_lighting(world: &mut TestWorld) {
  world.result = world
    .light
    .light(world.m, world.position, world.eyev, world.normalv, false);
}

#[then(regex = r#"^result = color\((.*), (.*), (.*)\)$"#)]
fn result_is_color(world: &mut TestWorld, r: f64, g: f64, b: f64) {
  assert_approx_eq!(world.result.0, r, 0.00001);
  assert_approx_eq!(world.result.1, g, 0.00001);
  assert_approx_eq!(world.result.2, b, 0.00001);
}

#[given(regex = r#"^eyev ← vector\(0, -√2/2, -√2/2\)$"#)]
fn eyev_is_vector3(world: &mut TestWorld) {
  world.eyev = Vector(0.0, -0.7071067811865475, -0.7071067811865475);
}

#[given(regex = r#"^in_shadow ← true$"#)]
fn in_shadow_is_true(world: &mut TestWorld) {
  world.in_shadow = true;
}

#[when(regex = r#"^result ← lighting\(m, light, position, eyev, normalv, in_shadow\)$"#)]
fn result_is_lighting_in_shadow(world: &mut TestWorld) {
  world.result = world
    .light
    .light(world.m, world.position, world.eyev, world.normalv, world.in_shadow);
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/materials.feature"));
}
