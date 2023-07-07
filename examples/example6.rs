use std::path::Path;

use sunhouse::canvas::Canvas;
use sunhouse::color::Color;
use sunhouse::hit::Hit;
use sunhouse::material::Material;

use sunhouse::point::Point;
use sunhouse::point_light::PointLight;
use sunhouse::ray::Ray;
use sunhouse::sphere::Sphere;

/// Build a world with a big purple sphere.
pub fn main() {
  /*
  ray_origin ← point(0, 0, -5)
  wall_z ← 10
  wall_size ← 7.0
  canvas ← canvas(canvas_pixels, canvas_pixels)
  color ← color(1, 0, 0) # red
  shape ← sphere()
  sphere.material ← material()
  sphere.material.color ← color(1, 0.2, 1)
  pixel_size ← wall_size / canvas_pixels
  half ← wall_size / 2
  light_position ← point(-10, 10, -10)
  light_color ← color(1, 1, 1)
  light ← point_light(light_position, light_color)
  # for each row of pixels in the canvas
  for y ← 0 to canvas_pixels - 1
    # compute the world y coordinate (top = +half, bottom = -half)
    world_y ← half - pixel_size * y
    # for each pixel in the row
    for x ← 0 to canvas_pixels - 1
      # compute the world x coordinate (left = -half, right = half)
      world_x ← -half + pixel_size * x
      # describe the point on the wall that the ray will target
      position ← point(world_x, world_y, wall_z)
      r ← ray(ray_origin, normalize(position - ray_origin))
      xs ← intersect(shape, r)
      if hit(xs) is defined
        write_pixel(canvas, x, y, color)
      end if
    end for
  end for
  */
  let ray_origin = Point(0.0, 0.0, -5.0);
  let _wall_z = 10.0;
  let wall_size = 7.0;
  let canvas_pixels = 256;
  let pixel_size = wall_size / (canvas_pixels as f64);
  let half = wall_size / 2.0;
  let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
  let _color = Color::new(1.0, 0.0, 0.0);
  let mut shape = Sphere::unit();
  let mut material = Material::default();
  material.color = Color::new(1.0, 0.2, 1.0);
  shape.material = material;
  let light_position = Point(-10.0, 10.0, -10.0);
  let light_color = Color::new(1.0, 1.0, 1.0);
  let light = PointLight::new(light_position, light_color);
  for y in 0..canvas_pixels {
    let world_y = half - pixel_size * (y as f64);
    for x in 0..canvas_pixels {
      let world_x = -half + pixel_size * (x as f64);
      let position = Point(world_x, world_y, 5.0);
      let r = Ray::new(ray_origin, (position - ray_origin).normalize());
      let xs = shape.intersect(r);
      if let Some(hit) = xs.hit() {
        let point = r.position(hit.t);
        let normal = hit.object.normal_at(point);
        let eye = -r.direction;
        let color = hit.object.material().lighting(light, point, eye, normal, false);
        canvas.set_color_at(x, y, color);
      } else {
        canvas.set_color_at(x, y, Color::default());
      }
    }
  }
  canvas.to_png(Path::new("examples/example6.png"));
}
