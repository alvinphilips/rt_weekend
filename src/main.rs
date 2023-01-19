use rayon::prelude::*;
use std::{sync::Arc, time::Instant};

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::*;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use utils::INFINITY;
use vec3::{Point3, Vec3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: usize = 1200;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 512;
const MAX_DEPTH: usize = 50;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color(0.0, 0.0, 0.0);
    }
    let mut record = HitRecord::default();
    if world.hit(ray, 0.001, INFINITY, &mut record) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if let Some(material) = record.clone().material {
            if material.scatter(ray, &record, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
        }
        return Color::default();
    }
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0)
}

struct Timer(Instant);

impl Timer {
    fn log(&self, message: &str) {
        eprintln!("[{:.02?}] - {message}", self.0.elapsed());
    }
}

fn main() {
    let timer = Timer(Instant::now());
    timer.log("Initializing Image");
    // Image
    // let mut image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    timer.log("Done.");

    // World
    let mut rng = rand::thread_rng();
    let mut world = HittableList::default();

    timer.log("Setting up World");

    // Materials
    let material_ground = Arc::new(Lambertian::new(&Color(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: u8 = rng.gen();
            let center = Point3(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material_sphere: Arc<dyn Material + Send + Sync> = match choose_mat {
                    0..=205 => {
                        let (color_1, color_2): (Color, Color) = rng.gen();
                        let albedo: Color = color_1 * color_2;
                        Arc::new(Lambertian::new(&albedo))
                    }
                    206..=243 => {
                        let albedo = Vec3::random_range(0.5, 1.0);
                        let fuzz = rng.gen::<f64>() / 2.0;
                        Arc::new(Metal::new(&albedo, fuzz))
                    }
                    _ => Arc::new(Dielectric::new(1.5)),
                };

                world.add(Arc::new(Sphere::new(center, 0.2, material_sphere)));
            }
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    let material_2 = Arc::new(Lambertian::new(&Color(0.4, 0.2, 0.1)));
    let material_3 = Arc::new(Metal::new(&Color(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(
        Point3(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));
    world.add(Arc::new(Sphere::new(
        Point3(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));
    world.add(Arc::new(Sphere::new(
        Point3(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let look_from = Point3(13.0, 2.0, 3.0);
    let look_at = Point3(0.0, 0.0, 0.0);

    let camera = Camera::new(look_from, look_at, Vec3::UP, 20.0, ASPECT_RATIO, 0.1, 10.0);

    let scale = 1.0 / SAMPLES_PER_PIXEL as f64;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let image: Vec<Vec3> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .map(|y| {
            let mut offsets = [0f64; SAMPLES_PER_PIXEL * 2];
            rand::thread_rng().fill(&mut offsets);
            let y = IMAGE_HEIGHT - y;
            let row: Vec<Vec3> = (0..IMAGE_WIDTH)
                .map(|x| {
                    let color: Color = (0..SAMPLES_PER_PIXEL)
                        .map(|sample| {
                            let u = (x as f64 + offsets[2 * sample]) / (IMAGE_WIDTH - 1) as f64;
                            let v =
                                (y as f64 + offsets[2 * sample + 1]) / (IMAGE_HEIGHT - 1) as f64;
                            let ray = camera.get_ray(u, v);
                            ray_color(&ray, &world, MAX_DEPTH)
                        })
                        .reduce(|accum, item| accum + item)
                        .unwrap();
                    color * scale
                })
                .collect();
            row
        })
        .flatten()
        .collect();
    for pixel in image {
        println!(
            "{} {} {}",
            (256.0 * utils::clamp(pixel.0.sqrt(), 0.0, 0.999)) as u8,
            (256.0 * utils::clamp(pixel.1.sqrt(), 0.0, 0.999)) as u8,
            (256.0 * utils::clamp(pixel.2.sqrt(), 0.0, 0.999)) as u8
        );
    }
}
