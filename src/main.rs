use std::rc::Rc;

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use image::Image;
use rand::random;
use ray::Ray;
use sphere::Sphere;
use utils::INFINITY;
use vec3::{Point3, Vec3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod image;
mod ray;
mod sphere;
mod utils;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut record = HitRecord::default();
    if world.hit(ray, 0.0, INFINITY, &mut record) {
        return 0.5 * (record.normal + Vec3(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * color::WHITE + t * Color(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let mut image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::default();

    // Render
    for y in 0..image.height {
        for x in 0..image.width {
            let mut pixel_color = Color::default();
            for _sample in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + random::<f64>()) / (image.width - 1) as f64;
                let v = (y as f64 + random::<f64>()) / (image.height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            image.set_pixel(pixel_color, x, y);
        }
    }

    image.print_ppm(SAMPLES_PER_PIXEL);
}
