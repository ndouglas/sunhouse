use std::path::Path;

use sunhouse::canvas::Canvas;
use sunhouse::color::Color;
use sunhouse::hit::Hit;

use sunhouse::point::Point;

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
  pixel_size ← wall_size / canvas_pixels
  half ← wall_size / 2
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
  let canvas_pixels = 1024;
  let pixel_size = wall_size / (canvas_pixels as f64);
  let half = wall_size / 2.0;
  let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
  let color = Color::new(1.0, 0.0, 0.0);
  let shape = Sphere::unit();
  for y in 0..canvas_pixels {
    let world_y = half - pixel_size * (y as f64);
    for x in 0..canvas_pixels {
      let world_x = -half + pixel_size * (x as f64);
      let position = Point(world_x, world_y, 5.0);
      let r = Ray::new(ray_origin, (position - ray_origin).normalize());
      let xs = shape.intersect(r);
      if let Some(_hit) = xs.hit() {
        canvas.set_color_at(x, y, color);
      } else {
        canvas.set_color_at(x, y, Color::default());
      }
    }
  }
  canvas.to_png(Path::new("examples/example5.png"));
}
