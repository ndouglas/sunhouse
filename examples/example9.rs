use std::f64::consts::PI;
use sunhouse::camera::Camera;
use sunhouse::color::Color;
use sunhouse::material::Material;
use sunhouse::matrix::Matrix;
use sunhouse::object::Object;
use sunhouse::plane::Plane;
use sunhouse::point::Point;
use sunhouse::point_light::PointLight;
use sunhouse::sphere::Sphere;
use sunhouse::vector::Vector;
use sunhouse::world::World;

/// Build a world with a floor, two walls, and three spheres.
pub fn main() {
  let mut world = World::empty();

  /*
    floor ← sphere()
    floor.transform ← scaling(10, 0.01, 10)
    floor.material ← material()
    floor.material.color ← color(1, 0.9, 0.9)
    floor.material.specular ← 0
  */
  let mut floor = Plane::default();
  floor.transform = Matrix::scaling(10.0, 0.01, 10.0);
  floor.material = Material::default();
  floor.material.color = Color::new(1.0, 0.9, 0.9);
  floor.material.specular = 0.0;

  /*
    middle ← sphere()
    middle.transform ← translation(-0.5, 1, 0.5)
    middle.material ← material()
    middle.material.color ← color(0.1, 1, 0.5)
    middle.material.diffuse ← 0.7
    middle.material.specular ← 0.3
  */
  let mut middle = Sphere::unit();
  middle.transform = Matrix::translation(-0.5, 1.0, 0.5);
  middle.material = Material::default();
  middle.material.color = Color::new(0.1, 1.0, 0.5);
  middle.material.diffuse = 0.7;
  middle.material.specular = 0.3;

  /*
    right ← sphere()
    right.transform ← translation(1.5, 0.5, -0.5)
      * scaling(0.5, 0.5, 0.5)
    right.material ← material()
    right.material.color ← color(0.5, 1, 0.1)
    right.material.diffuse ← 0.7
    right.material.specular ← 0.3
  */
  let mut right = Sphere::unit();
  right.transform = Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5);
  right.material = Material::default();
  right.material.color = Color::new(0.5, 1.0, 0.1);
  right.material.diffuse = 0.7;
  right.material.specular = 0.3;

  /*
    left ← sphere()
    left.transform ← translation(-1.5, 0.33, -0.75)
      * scaling(0.33, 0.33, 0.33)
    left.material ← material()
    left.material.color ← color(1, 0.8, 0.1)
    left.material.diffuse ← 0.7
    left.material.specular ← 0.3
  */
  let mut left = Sphere::unit();
  left.transform = Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33);
  left.material = Material::default();
  left.material.color = Color::new(1.0, 0.8, 0.1);
  left.material.diffuse = 0.7;
  left.material.specular = 0.3;

  /*
    world.light_source ← point_light(point(-10, 10, -10), color(1, 1, 1))
  */
  world
    .lights
    .push(PointLight::new((-10.0, 10.0, -10.0).into(), (1.0, 1.0, 1.0).into()));

  /*
    camera ← camera(100, 50, π/3)
    camera.transform ← view_transform(point(0, 1.5, -5), point(0, 1, 0), vector(0, 1, 0))
  */
  let mut camera = Camera::new(256, 256, PI / 3.0);
  camera.transform = Matrix::view_transform(Point(0.0, 1.5, -5.0), Point(0.0, 1.0, 0.0), Vector(0.0, 1.0, 0.0));

  world.objects.push(Object::Sphere(left));
  world.objects.push(Object::Sphere(right));
  world.objects.push(Object::Plane(floor));
  world.objects.push(Object::Sphere(middle));

  /*
    canvas ← render(camera, world)
  */
  world.render_png(&camera, "examples/example9.png");
}
