use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Sync>>,
}

impl HittableList {
    pub fn _clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        record: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}
