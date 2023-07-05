use cucumber::{given, then, when, World};
use sunhouse::color::Color;

use sunhouse::point::Point;
use sunhouse::point_light::PointLight;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  intensity: Color,
  position: Point,
  light: PointLight,
}

#[given(regex = r#"^intensity ← color\((.*), (.*), (.*)\)$"#)]
fn intensity_is_color(world: &mut TestWorld, r: f64, g: f64, b: f64) {
  world.intensity = Color(r, g, b);
}

#[given(regex = r#"^position ← point\((.*), (.*), (.*)\)$"#)]
fn position_is_point(world: &mut TestWorld, x: f64, y: f64, z: f64) {
  world.position = Point(x, y, z);
}

#[when(regex = r#"^light ← point_light\(position, intensity\)$"#)]
fn light_is_point_light(world: &mut TestWorld) {
  world.light = PointLight::new(world.position, world.intensity);
}

#[then(regex = r#"^light\.position = position$"#)]
fn light_position_is_position(world: &mut TestWorld) {
  assert_eq!(world.light.position, world.position);
}

#[then(regex = r#"^light\.intensity = intensity$"#)]
fn light_intensity_is_intensity(world: &mut TestWorld) {
  assert_eq!(world.light.intensity, world.intensity);
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/lights.feature"));
}
