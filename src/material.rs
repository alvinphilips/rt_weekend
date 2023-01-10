use std::fmt::Debug;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::NEAR_ZERO;
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
}

impl Metal {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
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
        *scattered = Ray::new(&record.point, &reflected);
        *attenuation = self.albedo;

        Vec3::dot(&scattered.direction, &record.normal) > 0.0
    }
}
