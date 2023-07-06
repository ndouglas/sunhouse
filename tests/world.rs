#[allow(clippy::too_many_arguments)]
use assert_approx_eq::assert_approx_eq;
use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use sunhouse::color::Color;
use sunhouse::comps::Comps;
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
  pub comps: Comps,
  pub c: Color,
  pub shape: Object,
  pub i: Intersection,
  pub outer_index: usize,
  pub inner_index: usize,
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

#[when(regex = r#"^c ← color_at\(w, r\)$"#)]
fn color_at(world: &mut TestWorld) {
  world.c = world.w.color_at(world.r);
}

#[given(regex = r#"^shape ← the first object in w$"#)]
fn shape_is_first_object(world: &mut TestWorld) {
  world.shape = world.w.objects[0];
}

#[given(regex = r#"^i ← intersection\((\d+\.?\d*), shape\)$"#)]
fn intersection_is(world: &mut TestWorld, t: f64) {
  world.i = Intersection::new(t, world.shape);
}

#[when(regex = r#"^comps ← prepare_computations\(i, r\)$"#)]
fn prepare_computations(world: &mut TestWorld) {
  world.comps = world.w.prepare_computations(world.i, world.r);
}

#[when(
  regex = r#"^w.light ← point_light\(point\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\), color\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)\)$"#
)]
#[given(
  regex = r#"^w.light ← point_light\(point\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\), color\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)\)$"#
)]
fn w_light_is(world: &mut TestWorld, x: f64, y: f64, z: f64, r: f64, g: f64, b: f64) {
  world.w.lights = vec![Point(x, y, z).into_light(Color::new(r, g, b))];
}

#[when(regex = r#"^c ← shade_hit\(w, comps\)$"#)]
fn shade_hit(world: &mut TestWorld) {
  world.c = world.w.shade_hit(world.comps);
}

#[then(regex = r#"^c = color\((-?\d+\.?\d*), (-?\d+\.?\d*), (-?\d+\.?\d*)\)$"#)]
fn c_is(world: &mut TestWorld, r: f64, g: f64, b: f64) {
  assert_approx_eq!(world.c.0, r, 1e-5);
  assert_approx_eq!(world.c.1, g, 1e-5);
  assert_approx_eq!(world.c.2, b, 1e-5);
}

#[given(regex = r#"^shape ← the second object in w$"#)]
fn shape_is_second_object(world: &mut TestWorld) {
  world.shape = world.w.objects[1];
}

#[given(regex = r#"^outer ← the first object in w$"#)]
fn outer_is_first_object(world: &mut TestWorld) {
  world.outer_index = 0;
}

#[given(regex = r#"^outer.material.ambient ← 1$"#)]
fn outer_material_ambient(world: &mut TestWorld) {
  // Update the sphere's material's ambient property.
  if let Object::Sphere(ref mut sphere) = &mut world.w.objects[world.outer_index] {
    sphere.material.ambient = 1.0;
  }
}

#[given(regex = r#"^inner ← the second object in w$"#)]
fn inner_is_second_object(world: &mut TestWorld) {
  world.inner_index = 1;
}

#[given(regex = r#"^inner.material.ambient ← 1$"#)]
fn inner_material_ambient(world: &mut TestWorld) {
  if let Object::Sphere(ref mut sphere) = &mut world.w.objects[world.inner_index] {
    sphere.material.ambient = 1.0;
  }
}

#[then(regex = r#"^c = inner.material.color$"#)]
fn c_is_inner_material_color(world: &mut TestWorld) {
  if let Object::Sphere(sphere) = &world.w.objects[world.inner_index] {
    assert_eq!(world.c, sphere.material.color);
  }
}

// This runs before everything else, so you can setup things here.
fn main() {
  futures::executor::block_on(TestWorld::run("tests/features/world.feature"));
}
