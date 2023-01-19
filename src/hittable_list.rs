use crate::{
    hittable::{HitRecord, Hittable},
    sphere::Sphere,
};

#[derive(Debug, Default)]
pub struct HittableList {
    objects: Vec<Sphere>,
}

unsafe impl Sync for HittableList {}

impl HittableList {
    pub fn _clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Sphere) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}
