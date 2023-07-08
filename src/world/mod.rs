use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::comps::Comps;
use crate::hit::Hit;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::matrix::Matrix4x4;
use crate::object::Object;
use crate::point::Point;
use crate::point_light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;

/// The world struct.
#[derive(Debug, Clone)]
pub struct World {
  /// The objects in the world.
  pub objects: Vec<Object>,
  /// The lights in the world.
  pub lights: Vec<PointLight>,
}

impl World {
  pub fn empty() -> Self {
    World::new(vec![], vec![])
  }

  /// Create a new world.
  pub fn new(objects: Vec<Object>, lights: Vec<PointLight>) -> Self {
    World { objects, lights }
  }

  /// Calculate the intersections between the world and the given ray as
  /// a collection of intersections.
  pub fn intersect(&mut self, ray: Ray) -> Vec<Intersection> {
    let mut intersections = vec![];
    for object in &mut self.objects {
      intersections.append(&mut object.intersect(ray));
    }
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    intersections
  }

  /// Prepare computations.
  pub fn prepare_computations(&self, intersection: Intersection, ray: Ray) -> crate::comps::Comps {
    Comps::prepare(intersection, ray)
  }

  /// Calculate the color at the intersection encapsulated by comps.
  pub fn shade_hit(&mut self, comps: Comps) -> Color {
    let in_shadow = self.is_shadowed(comps.over_point);
    // Iterate over the lights in the world, calculating the color at the
    // intersection for each light.
    self.lights.iter().fold(Color::new(0.0, 0.0, 0.0), |acc, light| {
      acc
        + comps
          .object
          .material()
          .lighting(*light, comps.point, comps.eyev, comps.normalv, in_shadow)
    })
  }

  /// Calculate the color at the ray.
  pub fn color_at(&mut self, ray: Ray) -> Color {
    let intersections = self.intersect(ray);
    // Find the hit, if any.
    let hit = intersections.hit();
    // If there was no hit, return black.
    if hit.is_none() {
      return Color::new(0.0, 0.0, 0.0);
    }
    // Otherwise, calculate the color at the hit.
    let hit = hit.unwrap();
    let comps = self.prepare_computations(hit, ray);

    self.shade_hit(comps)
  }

  /// Render the world.
  pub fn render(&mut self, camera: &Camera) -> Canvas {
    camera.render(self)
  }

  /// Render the world as a PNG.
  pub fn render_png(&mut self, camera: &Camera, filename: &str) {
    camera.render_png(self, filename)
  }

  /// Determine if the given point is in shadow.
  pub fn is_shadowed(&mut self, point: Point) -> bool {
    let v = self.lights[0].position - point;
    let distance = v.magnitude();
    let direction = v.normalize();
    let r = Ray::new(point, direction);
    let intersections = self.intersect(r);
    let hit = intersections.hit();
    if hit.is_none() {
      return false;
    }
    let hit = hit.unwrap();
    hit.t < distance
  }
}

impl Default for World {
  fn default() -> Self {
    World {
      objects: vec![
        Object::Sphere(Sphere {
          center: Point::default(),
          radius: 1.0,
          transform: Matrix::identity(),
          material: Material {
            color: (0.8, 1.0, 0.6).into(),
            diffuse: 0.7,
            specular: 0.2,
            ..Material::default()
          },
        }),
        Object::Sphere(Sphere {
          center: Point::default(),
          radius: 1.0,
          transform: Matrix::Matrix4x4(Matrix4x4([
            [0.5, 0.0, 0.0, 0.0],
            [0.0, 0.5, 0.0, 0.0],
            [0.0, 0.0, 0.5, 0.0],
            [0.0, 0.0, 0.0, 1.0],
          ])),
          material: Material {
            color: (1.0, 1.0, 1.0).into(),
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            ..Material::default()
          },
        }),
      ],
      lights: vec![PointLight::new((-10.0, 10.0, -10.0).into(), (1.0, 1.0, 1.0).into())],
    }
  }
}
