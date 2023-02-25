use assert_approx_eq::assert_approx_eq;
use cucumber::{given, then, World};
use sunhouse::point::Point;
use sunhouse::tuple::Tuple;
use sunhouse::vector::Vector;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TuplesWorld {
  pub a: Tuple,
  pub a1: Tuple,
  pub a2: Tuple,
  pub t1: Tuple,
  pub t2: Tuple,
  pub p: Tuple,
  pub v: Tuple,
}

#[given(regex = r"^(a|a1|a2) ← tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn set_a(world: &mut TuplesWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
  match name.as_str() {
    "a1" => world.a1 = Tuple::new(x, y, z, w),
    "a2" => world.a2 = Tuple::new(x, y, z, w),
    _ => world.a = Tuple::new(x, y, z, w),
  }
}

#[given(regex = r"^(t1|t2) ← (point|vector)\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn set_t(world: &mut TuplesWorld, name: String, r#type: String, x: f64, y: f64, z: f64) {
  match (name.as_str(), r#type.as_str()) {
    ("t1", "point") => world.t1 = Tuple::Point(Point(x, y, z)),
    ("t2", "point") => world.t2 = Tuple::Point(Point(x, y, z)),
    ("t1", "vector") => world.t1 = Tuple::Vector(Vector(x, y, z)),
    ("t2", "vector") => world.t2 = Tuple::Vector(Vector(x, y, z)),
    _ => unreachable!("This should not happen!"),
  }
}

#[given(expr = r"p ← point\({float}, {float}, {float}\)")]
fn set_p(world: &mut TuplesWorld, x: f64, y: f64, z: f64) {
  world.p = Tuple::Point(Point(x, y, z));
}

#[then(regex = r"^(a|a1|a2|t1|t2) ([\+-]) (a|a1|a2|t1|t2) = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn x_op_y_equals_z(world: &mut TuplesWorld, name1: String, op: String, name2: String, x: f64, y: f64, z: f64, w: f64) {
  let lhs = match name1.as_str() {
    "a" => world.a,
    "a1" => world.a1,
    "a2" => world.a2,
    "t1" => world.t1,
    "t2" => world.t2,
    _ => unreachable!("This should not happen!"),
  };
  let rhs = match name2.as_str() {
    "a" => world.a,
    "a1" => world.a1,
    "a2" => world.a2,
    "t1" => world.t1,
    "t2" => world.t2,
    _ => unreachable!("This should not happen!"),
  };
  assert!(lhs.is_point() || lhs.is_vector());
  assert!(rhs.is_point() || rhs.is_vector());
  let result = match (op.as_str(), lhs, rhs) {
    (_, Tuple::None, _) => unreachable!("lhs is None"),
    (_, _, Tuple::None) => unreachable!("rhs is None"),
    ("+", Tuple::Point(_), Tuple::Point(_)) => panic!("Cannot add two points"),
    ("-", Tuple::Point(lhs), Tuple::Point(rhs)) => Tuple::Vector(lhs - rhs),
    ("+", Tuple::Point(lhs), Tuple::Vector(rhs)) => Tuple::Point(lhs + rhs),
    ("-", Tuple::Point(lhs), Tuple::Vector(rhs)) => Tuple::Point(lhs - rhs),
    ("+", Tuple::Vector(lhs), Tuple::Point(rhs)) => Tuple::Point(lhs + rhs),
    ("-", Tuple::Vector(lhs), Tuple::Point(rhs)) => Tuple::Point(lhs - rhs),
    ("+", Tuple::Vector(lhs), Tuple::Vector(rhs)) => Tuple::Vector(lhs + rhs),
    ("-", Tuple::Vector(lhs), Tuple::Vector(rhs)) => Tuple::Vector(lhs - rhs),
    (_, _, _) => unreachable!("Unknown operation: {} {} {}", name1, op, name2),
  };
  assert_eq!(result, Tuple::new(x, y, z, w));
}

#[then(regex = r"^(a|a1|a2|t1|t2) \* (-?\d+.?\d*) = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn x_mul_y_equals_z(world: &mut TuplesWorld, name: String, rhs: f64, x: f64, y: f64, z: f64, w: f64) {
  let lhs = match name.as_str() {
    "a" => world.a,
    "a1" => world.a1,
    "a2" => world.a2,
    "t1" => world.t1,
    "t2" => world.t2,
    _ => unreachable!("This should not happen!"),
  };
  assert!(lhs.is_point() || lhs.is_vector());
  assert_eq!(lhs * rhs, Tuple::new(x, y, z, w));
}

#[then(regex = r"^(a|a1|a2|t1|t2) / (-?\d+.?\d*) = tuple\((-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*), (-?\d+.?\d*)\)$")]
fn x_div_y_equals_z(world: &mut TuplesWorld, name: String, rhs: f64, x: f64, y: f64, z: f64, w: f64) {
  let lhs = match name.as_str() {
    "a" => world.a,
    "a1" => world.a1,
    "a2" => world.a2,
    "t1" => world.t1,
    "t2" => world.t2,
    _ => unreachable!("This should not happen!"),
  };
  assert!(lhs.is_point() || lhs.is_vector());
  assert_eq!(lhs / rhs, Tuple::new(x, y, z, w));
}

#[then(expr = r"-a = tuple\({float}, {float}, {float}, {float}\)")]
fn check_neg_a_equality(world: &mut TuplesWorld, x: f64, y: f64, z: f64, w: f64) {
  assert_eq!(-world.a, Tuple::new(x, y, z, w), "{:?} = tuple({}, {}, {}, {})", -world.a, x, y, z, w); 
}

#[then(expr = r"p = tuple\({float}, {float}, {float}, {float}\)")]
fn check_p_equality(world: &mut TuplesWorld, x: f64, y: f64, z: f64, w: f64) {
  assert_eq!(world.p, Tuple::new(x, y, z, w), "{:?} = tuple({}, {}, {}, {})", world.p, x, y, z, w); 
}

#[given(expr = r"v ← vector\({float}, {float}, {float}\)")]
fn set_v(world: &mut TuplesWorld, x: f64, y: f64, z: f64) {
  world.v = Tuple::Vector(Vector(x, y, z));
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
fn a_is_a(world: &mut TuplesWorld, not: String, r#type: String) {
  let is_a = not == "";
  if is_a {
    match r#type.as_str() {
      "point" => assert!(world.a.is_point(),  "a is not a point when it should be"),
      "vector" => assert!(world.a.is_vector(), "a is not a vector when it should be"),
      _ => panic!("Unknown type: {}", r#type),
    }  
  }
  else {
    match r#type.as_str() {
      "point" => assert!(!world.a.is_point(),  "a is a point when it should not be"),
      "vector" => assert!(!world.a.is_vector(), "a is a vector when it should not be"),
      _ => panic!("Unknown type: {}", r#type),
    }  
  }
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TuplesWorld::run("tests/features/tuples.feature"));
}
