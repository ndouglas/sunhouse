use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::matrix::Matrix4x4;
use crate::object::Object;
use crate::point::Point;
use crate::point_light::PointLight;
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
  pub fn intersect(&self, ray: crate::ray::Ray) -> Vec<Intersection> {
    let mut intersections = vec![];
    for object in &self.objects {
      intersections.append(&mut object.intersect(ray));
    }
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    intersections
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
