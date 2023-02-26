#[allow(clippy::too_many_arguments)]
use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use sunhouse::canvas::Canvas;
use sunhouse::color::Color;

// `CanvasWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct CanvasWorld {
  c: Canvas,
  c1: Color,
  c2: Color,
  c3: Color,
  red: Color,
  ppm: String,
}

#[given(regex = r"^c ← canvas\((\d+), (\d+)\)$")]
fn set_c(world: &mut CanvasWorld, width: usize, height: usize) {
  world.c = Canvas::new(width, height);
}

#[given(regex = r"^red ← color\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn set_red(world: &mut CanvasWorld, r: f64, g: f64, b: f64) {
  world.red = Color(r, g, b);
}

#[given(regex = r"^(c1|c2|c3) ← color\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn set_cx(world: &mut CanvasWorld, color_id: String, r: f64, g: f64, b: f64) {
  match color_id.as_str() {
    "c1" => world.c1 = Color(r, g, b),
    "c2" => world.c2 = Color(r, g, b),
    "c3" => world.c3 = Color(r, g, b),
    _ => panic!("Unknown color: {}", color_id),
  }
}

#[when(regex = r"^write_pixel\((c), (\d+), (\d+), (red|c1|c2|c3)\)$")]
fn write_pixel(world: &mut CanvasWorld, canvas_id: String, x: usize, y: usize, color_id: String) {
  let canvas = match canvas_id.as_str() {
    "c" => &mut world.c,
    _ => panic!("Unknown canvas: {}", canvas_id),
  };
  let color = match color_id.as_str() {
    "red" => world.red,
    "c1" => world.c1,
    "c2" => world.c2,
    "c3" => world.c3,
    _ => panic!("Unknown color: {}", color_id),
  };
  canvas.set_color_at(x, y, color);
}

#[when(regex = r"^ppm ← canvas_to_ppm\((c)\)$")]
fn canvas_to_ppm(world: &mut CanvasWorld, canvas_id: String) {
  let canvas = match canvas_id.as_str() {
    "c" => &world.c,
    _ => panic!("Unknown canvas: {}", canvas_id),
  };
  let ppm = canvas.to_ppm();
  world.ppm = ppm;
}

#[then(expr = r"lines 1-3 of ppm are")]
fn check_ppm_header(world: &mut CanvasWorld, step: &Step) {
  let lines: Vec<&str> = world.ppm.lines().collect();
  let expected_lines: Vec<&str> = step
    .docstring
    .as_ref()
    .expect("No docstring found")
    .lines()
    .skip(1)
    .collect();
  assert_eq!(lines[0], expected_lines[0]);
  assert_eq!(lines[1], expected_lines[1]);
  assert_eq!(lines[2], expected_lines[2]);
}

#[then(expr = r"lines 4-6 of ppm are")]
fn check_ppm_body(world: &mut CanvasWorld, step: &Step) {
  let lines: Vec<&str> = world.ppm.lines().collect();
  let expected_lines: Vec<&str> = step
    .docstring
    .as_ref()
    .expect("No docstring found")
    .lines()
    .skip(1)
    .collect();
  assert_eq!(lines[3], expected_lines[0]);
  assert_eq!(lines[4], expected_lines[1]);
  assert_eq!(lines[5], expected_lines[2]);
}

#[then(regex = r"^c.(width|height) = (\d+)$")]
fn check_canvas_property(world: &mut CanvasWorld, property: String, value: usize) {
  match property.as_str() {
    "width" => assert_eq!(world.c.width, value),
    "height" => assert_eq!(world.c.height, value),
    _ => panic!("Unknown property: {}", property),
  }
}

#[then(regex = r"^pixel_at\((c), (\d+), (\d+)\) = (red)$")]
fn check_pixel_at(world: &mut CanvasWorld, canvas_id: String, x: usize, y: usize, color: String) {
  let canvas = match canvas_id.as_str() {
    "c" => &world.c,
    _ => panic!("Unknown canvas: {}", canvas_id),
  };
  let expected = match color.as_str() {
    "red" => world.red,
    _ => panic!("Unknown color: {}", color),
  };
  assert_eq!(canvas.get_color_at(x, y), expected);
}

#[then(regex = r"^every pixel of (c) is color\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn get_every_pixel(world: &mut CanvasWorld, canvas_id: String, r: f64, g: f64, b: f64) {
  let canvas = match canvas_id.as_str() {
    "c" => &world.c,
    _ => panic!("Unknown canvas: {}", canvas_id),
  };
  let expected = Color(r, g, b);
  for y in 0..canvas.height {
    for x in 0..canvas.width {
      assert_eq!(canvas.get_color_at(x, y), expected);
    }
  }
}

#[then(expr = "ppm ends with a newline character")]
fn check_ppm_newline(world: &mut CanvasWorld) {
  assert!(world.ppm.ends_with('\n'));
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(CanvasWorld::run("tests/features/canvas.feature"));
}
