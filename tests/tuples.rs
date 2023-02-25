use assert_approx_eq::assert_approx_eq;
use cucumber::{given, then, World};
use sunhouse::point::Point;
use sunhouse::vector::Vector;

// Distinguish between points and vectors.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Tuple {
  Point(Point),
  Vector(Vector),
  #[default]
  None,
}

impl Tuple {
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
    if w > 0.5 {
      Tuple::Point(Point(x, y, z))
    } else {
      Tuple::Vector(Vector(x, y, z))
    }
  }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TuplesWorld {
  pub a: Tuple,
  pub p: Tuple,
  pub v: Tuple,
}

#[given(expr = r"a ← tuple\({float}, {float}, {float}, {float}\)")]
fn set_a(world: &mut TuplesWorld, x: f64, y: f64, z: f64, w: f64) {
  world.a = Tuple::new(x, y, z, w);
}

#[given(expr = r"p ← point\({float}, {float}, {float}\)")]
fn set_p(world: &mut TuplesWorld, x: f64, y: f64, z: f64) {
  world.p = Tuple::new(x, y, z, 1.0);
}

#[then(expr = r"p = tuple\({float}, {float}, {float}, {float}\)")]
fn check_p_equality(world: &mut TuplesWorld, x: f64, y: f64, z: f64, w: f64) {
  assert_eq!(world.p, Tuple::new(x, y, z, w), "{:?} = tuple({}, {}, {}, {})", world.p, x, y, z, w); 
}

#[given(expr = r"v ← vector\({float}, {float}, {float}\)")]
fn set_v(world: &mut TuplesWorld, x: f64, y: f64, z: f64) {
  world.v = Tuple::new(x, y, z, 0.0);
}

#[then(expr = r"v = tuple\({float}, {float}, {float}, {float}\)")]
fn check_v_equality(world: &mut TuplesWorld, x: f64, y: f64, z: f64, w: f64) {
  assert_eq!(world.v, Tuple::new(x, y, z, w), "{:?} = tuple({}, {}, {}, {})", world.p, x, y, z, w); 
}

#[then(regex = r"^a.(w|x|y|z) = (-?\d+.?\d*)$")]
fn tuple_property_equals(world: &mut TuplesWorld, key: String, value: f64) {
  let (x, y, z, w) = match world.a {
    Tuple::Point(Point(x, y, z)) => (x, y, z, 1.0),
    Tuple::Vector(Vector(x, y, z)) => (x, y, z, 0.0),
    Tuple::None => (0.0, 0.0, 0.0, 0.0),
  };
  match key.as_str() {
    "x" => assert_approx_eq!(x, value, 0.001),
    "y" => assert_approx_eq!(y, value, 0.001),
    "z" => assert_approx_eq!(z, value, 0.001),
    "w" => assert_approx_eq!(w, value, 0.001),
    _ => panic!("Unknown key: {}", key),
  }
}

#[then(regex = r"a is( not)? a (point|vector)")]
fn a_is_a(_world: &mut TuplesWorld, not: String, r#type: String) {
  let is_a = not == "not";
  match r#type.as_str() {
    "point" => assert!(!is_a,  "a is {} a point", not),
    "vector" => assert!(!is_a, "a is {} a vector", not),
    _ => panic!("Unknown type: {}", r#type),
  }
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TuplesWorld::run("tests/features/tuples.feature"));
}
