#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;
use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use sunhouse::comps::Comps;
use sunhouse::hit::Hit;
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
  pub s: Sphere,
  pub i: Option<Intersection>,
  pub r: Ray,
  pub shape: Object,
  pub comps: Comps,
  pub i1: Intersection,
  pub i2: Intersection,
  pub i3: Intersection,
  pub i4: Intersection,
  pub xs: Vec<Intersection>,
}

#[given(regex = r#"^s ← sphere\(\)$"#)]
fn sphere_is(world: &mut TestWorld) {
  world.s = Sphere::unit();
}

#[when(regex = r#"^i ← intersection\((\d+\.?\d*), s\)$"#)]
fn intersection_is(world: &mut TestWorld, t: f64) {
  world.i = Some(Intersection::new(t, Object::Sphere(world.s.clone())));
}

#[then(regex = r#"^i\.t = (\d+\.?\d*)$"#)]
fn intersection_t_is(world: &mut TestWorld, t: f64) {
  assert_approx_eq!(world.i.as_ref().unwrap().t, t);
}

#[then(regex = r#"^i\.object = s$"#)]
fn intersection_object_is(world: &mut TestWorld) {
  assert_eq!(world.i.as_ref().unwrap().object, Object::Sphere(world.s.clone()));
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
  world.i = Some(Intersection::new(t, world.shape.clone()));
}

#[when(regex = r#"^comps ← prepare_computations\(i, r\)$"#)]
fn prepare_computations(world: &mut TestWorld) {
  world.comps = Comps::prepare(&world.i.as_ref().unwrap(), world.r);
}

#[then(regex = r#"^comps\.t = i\.t$"#)]
fn comps_t_is(world: &mut TestWorld) {
  assert_approx_eq!(world.comps.t, world.i.as_ref().unwrap().t);
}

#[then(regex = r#"^comps\.object = i\.object$"#)]
fn comps_object_is(world: &mut TestWorld) {
  assert_eq!(world.comps.object, world.i.as_ref().unwrap().object);
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

#[given(regex = r#"^i1 ← intersection\((-?\d+\.?\d*), s\)$"#)]
fn i1_is(world: &mut TestWorld, t: f64) {
  world.i1 = Intersection::new(t, Object::Sphere(world.s.clone()));
}

#[given(regex = r#"^i2 ← intersection\((-?\d+\.?\d*), s\)$"#)]
fn i2_is(world: &mut TestWorld, t: f64) {
  world.i2 = Intersection::new(t, Object::Sphere(world.s.clone()));
}

#[given(regex = r#"^xs ← intersections\(i1, i2\)$"#)]
#[when(regex = r#"^xs ← intersections\(i1, i2\)$"#)]
fn xs_is(world: &mut TestWorld) {
  world.xs = vec![world.i1.clone(), world.i2.clone()];
}

#[given(regex = r#"^xs ← intersections\(i2, i1\)$"#)]
fn xs_is_reversed(world: &mut TestWorld) {
  world.xs = vec![world.i2.clone(), world.i1.clone()];
}

#[when(regex = r#"^i ← hit\(xs\)$"#)]
fn i_is_hit(world: &mut TestWorld) {
  world.i = world.xs.hit();
}

#[then(regex = r#"^xs.count = (\d+)$"#)]
fn xs_count_is(world: &mut TestWorld, count: usize) {
  assert_eq!(world.xs.len(), count);
}

#[then(regex = r#"^i is nothing$"#)]
fn i_is_nothing(world: &mut TestWorld) {
  assert!(world.i.is_none());
}

#[then(regex = r#"^i = i1$"#)]
fn i_is_i1(world: &mut TestWorld) {
  assert_eq!(world.i, Some(world.i1.clone()));
}

#[then(regex = r#"^i = i2$"#)]
fn i_is_i2(world: &mut TestWorld) {
  assert_eq!(world.i, Some(world.i2.clone()));
}

#[given(regex = r#"^i3 ← intersection\((-?\d+\.?\d*), s\)$"#)]
fn i3_is(world: &mut TestWorld, t: f64) {
  world.i3 = Intersection::new(t, Object::Sphere(world.s.clone()));
}

#[given(regex = r#"^i4 ← intersection\((-?\d+\.?\d*), s\)$"#)]
fn i4_is(world: &mut TestWorld, t: f64) {
  world.i4 = Intersection::new(t, Object::Sphere(world.s.clone()));
}

#[given(regex = r#"^xs ← intersections\(i1, i2, i3, i4\)$"#)]
fn xs_is_many(world: &mut TestWorld) {
  world.xs = vec![world.i1.clone(), world.i2.clone(), world.i3.clone(), world.i4.clone()];
}

#[then(regex = r#"^i = i3$"#)]
fn i_is_i3(world: &mut TestWorld) {
  assert_eq!(world.i, Some(world.i3.clone()));
}

#[then(regex = r#"^i = i4$"#)]
fn i_is_i4(world: &mut TestWorld) {
  assert_eq!(world.i, Some(world.i4.clone()));
}

#[given(regex = r#"^shape ← sphere\(\) with:$"#)]
fn shape_is_with(world: &mut TestWorld, step: &Step) {
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter().skip(1) {
      let key = line[0].to_string();
      let value = line[1].to_string();
      match key.as_str() {
        "transform" => {
          let transform = if value.contains("scaling") {
            let stripped = value.replace("scaling(", "").replace(')', "");
            let mut values = stripped.split(',').map(|s| s.trim());
            let x = values.next().unwrap().parse::<f64>().unwrap();
            let y = values.next().unwrap().parse::<f64>().unwrap();
            let z = values.next().unwrap().parse::<f64>().unwrap();
            Matrix::scaling(x, y, z)
          } else if value.contains("translation") {
            let stripped = value.replace("translation(", "").replace(')', "");
            let mut values = stripped.split(',').map(|s| s.trim());
            let x = values.next().unwrap().parse::<f64>().unwrap();
            let y = values.next().unwrap().parse::<f64>().unwrap();
            let z = values.next().unwrap().parse::<f64>().unwrap();
            Matrix::translation(x, y, z)
          } else {
            panic!("Unknown transform: {}", value);
          };
          let mut sphere = Sphere::default();
          sphere.transform = transform;
          world.shape = Object::Sphere(sphere);
        },
        _ => panic!("Unknown key: {}", key),
      }
    }
  }
}

#[then(regex = r#"^comps.over_point.z < -EPSILON/2$"#)]
fn comps_over_point_z_is(world: &mut TestWorld) {
  assert!(world.comps.over_point.2 < -0.0001 / 2.0);
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/intersections.feature"));
}
