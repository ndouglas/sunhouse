use cucumber::{given, then, World};

use sunhouse::matrix::Matrix;
use sunhouse::object::Object;

use sunhouse::test_shape::TestShape;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  pub s: Object,
}

#[given(regex = r#"^s ← test_shape\(\)$"#)]
fn test_shape_is(world: &mut TestWorld) {
  world.s = Object::TestShape(TestShape::default());
}

#[then(regex = r#"^s\.transform = identity_matrix$"#)]
fn test_shape_transform_is_identity_matrix(world: &mut TestWorld) {
  assert_eq!(world.s.transform(), Matrix::identity());
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/shapes.feature"));
}
