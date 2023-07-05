#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;
use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use sunhouse::color::Color;
use sunhouse::intersection::Intersection;
use sunhouse::object::Object;
use sunhouse::point::Point;
use sunhouse::point_light::PointLight;
use sunhouse::ray::Ray;
use sunhouse::sphere::Sphere;

use sunhouse::vector::Vector;
use sunhouse::world::World as RenderWorld;

// `TestWorld` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TestWorld {
  pub w: RenderWorld,
  pub light: PointLight,
  pub r: Ray,
  pub s1: Sphere,
  pub s2: Sphere,
  pub xs: Vec<Intersection>,
}

#[given(regex = r#"^w ← world\(\)$"#)]
fn world_is(world: &mut TestWorld) {
  world.w = RenderWorld::empty();
}

#[when(regex = r#"^w ← default_world\(\)$"#)]
#[given(regex = r#"^w ← default_world\(\)$"#)]
fn world_is_default(world: &mut TestWorld) {
  world.w = RenderWorld::default();
}

#[then(regex = r#"^w contains no objects$"#)]
fn world_contains_no_objects(world: &mut TestWorld) {
  assert_eq!(world.w.objects.len(), 0);
}

#[then(regex = r#"^w has no light source$"#)]
fn world_has_no_light_source(world: &mut TestWorld) {
  assert_eq!(world.w.lights.len(), 0);
}

#[then(regex = r#"^w\.light = light$"#)]
fn world_light_is(world: &mut TestWorld) {
  assert_eq!(world.w.lights[0], world.light);
}

#[given(
  regex = r#"^light ← point_light\(point\((-?\d+\.?\d*), (-?\d+\.?\d*), (-\d+\.?\d*)\), color\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)\)$"#
)]
fn light_is(world: &mut TestWorld, x: f64, y: f64, z: f64, r: f64, g: f64, b: f64) {
  world.light = Point(x, y, z).into_light(Color::new(r, g, b));
}

#[given(regex = r#"^(s1|s2) ← sphere\(\) with:$"#)]
fn sphere_is(world: &mut TestWorld, sid: String, step: &Step) {
  if let Some(table) = step.table.as_ref() {
    for line in table.rows.iter() {
      let key = line[0].to_string();
      let value = line[1].to_string();
      match key.as_str() {
        "material.color" => {
          let mut color = Color::default();
          // Decode a tuple of 3 floats.
          let stripped = value.replace(['(', ')'], "");
          // Split into three values via comma and trim whitespace, then collect into a Vec.
          let mut values = stripped.split(',').map(|s| s.trim());
          color.0 = values.next().unwrap().parse::<f64>().unwrap();
          color.1 = values.next().unwrap().parse::<f64>().unwrap();
          color.2 = values.next().unwrap().parse::<f64>().unwrap();
          match sid.as_str() {
            "s1" => world.s1.material.color = color,
            "s2" => world.s2.material.color = color,
            _ => panic!("Unknown sphere: {}", sid),
          }
        },
        "material.ambient" => match sid.as_str() {
          "s1" => world.s1.material.ambient = value.parse::<f64>().unwrap(),
          "s2" => world.s2.material.ambient = value.parse::<f64>().unwrap(),
          _ => panic!("Unknown sphere: {}", sid),
        },
        "material.diffuse" => match sid.as_str() {
          "s1" => world.s1.material.diffuse = value.parse::<f64>().unwrap(),
          "s2" => world.s2.material.diffuse = value.parse::<f64>().unwrap(),
          _ => panic!("Unknown sphere: {}", sid),
        },
        "material.specular" => match sid.as_str() {
          "s1" => world.s1.material.specular = value.parse::<f64>().unwrap(),
          "s2" => world.s2.material.specular = value.parse::<f64>().unwrap(),
          _ => panic!("Unknown sphere: {}", sid),
        },
        "material.shininess" => match sid.as_str() {
          "s1" => world.s1.material.shininess = value.parse::<f64>().unwrap(),
          "s2" => world.s2.material.shininess = value.parse::<f64>().unwrap(),
          _ => panic!("Unknown sphere: {}", sid),
        },
        "transform" => {
          let transform = if value.contains("scaling") {
            let stripped = value.replace("scaling(", "").replace(')', "");
            let mut values = stripped.split(',').map(|s| s.trim());
            let x = values.next().unwrap().parse::<f64>().unwrap();
            let y = values.next().unwrap().parse::<f64>().unwrap();
            let z = values.next().unwrap().parse::<f64>().unwrap();
            sunhouse::matrix::Matrix::scaling(x, y, z)
          } else if value.contains("translation") {
            let stripped = value.replace("translation(", "").replace(')', "");
            let mut values = stripped.split(',').map(|s| s.trim());
            let x = values.next().unwrap().parse::<f64>().unwrap();
            let y = values.next().unwrap().parse::<f64>().unwrap();
            let z = values.next().unwrap().parse::<f64>().unwrap();
            sunhouse::matrix::Matrix::translation(x, y, z)
          } else {
            panic!("Unknown transform: {}", value);
          };
          match sid.as_str() {
            "s1" => world.s1.transform = transform,
            "s2" => world.s2.transform = transform,
            _ => panic!("Unknown sphere: {}", sid),
          }
        },
        _ => panic!("Unknown key: {}", key),
      }
    }
  }
}

#[then(regex = r#"^w contains s1$"#)]
fn world_contains_s1(world: &mut TestWorld) {
  assert_eq!(world.w.objects[0], Object::Sphere(world.s1));
}

#[then(regex = r#"^w contains s2$"#)]
fn world_contains_s2(world: &mut TestWorld) {
  assert_eq!(world.w.objects[1], Object::Sphere(world.s2));
}

#[given(
  regex = r#"^r ← ray\(point\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\), vector\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)\)$"#
)]
fn ray_is(world: &mut TestWorld, x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) {
  world.r = Ray::new(Point(x, y, z), Vector(vx, vy, vz));
}

#[when(regex = r#"^xs ← intersect_world\(w, r\)$"#)]
fn intersect_world(world: &mut TestWorld) {
  world.xs = world.w.intersect(world.r);
}

#[then(regex = r#"^xs\.count = (\d+)$"#)]
fn xs_count(world: &mut TestWorld, count: usize) {
  assert_eq!(world.xs.len(), count);
}

#[then(regex = r#"^xs\[(\d+)\].t = (\d+\.?\d*)$"#)]
fn xs_t(world: &mut TestWorld, index: usize, t: f64) {
  assert_approx_eq!(world.xs[index].t, t);
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/world.feature"));
}
