use std::fmt::Debug;

use rand::random;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::{clamp, NEAR_ZERO};
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Material instance")
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        if scatter_direction.magnitude_squared() < NEAR_ZERO {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(&record.point, &scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz: clamp(fuzz, 0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&ray_in.direction.normalized(), &record.normal);
        *scattered = Ray::new(
            &record.point,
            &(reflected + self.fuzz * Vec3::random_in_unit_sphere()),
        );
        *attenuation = self.albedo;

        Vec3::dot(&scattered.direction, &record.normal) > 0.0
    }
}

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, reference_idx: f64) -> f64 {
        let r0 = (1.0 - reference_idx) / (1.0 + reference_idx);
        let r0 = r0 * r0;
        r0 + (1.0 * r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.direction.normalized();

        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_retract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_retract || Self::reflectance(cos_theta, refraction_ratio) > random::<f64>() {
                Vec3::reflect(&unit_direction, &record.normal)
            } else {
                Vec3::refract(&unit_direction, &record.normal, refraction_ratio)
            };

        *scattered = Ray::new(&record.point, &direction);

        true
    }
}
