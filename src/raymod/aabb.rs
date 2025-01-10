use crate::raymod::*;

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let min = Vec3::new(
        f64::min(box0.min.x, box1.min.x),
        f64::min(box0.min.y, box1.min.y),
        f64::min(box0.min.z, box1.min.z));
    let max = Vec3::new(
        f64::max(box0.max.x, box1.max.x),
        f64::max(box0.max.y, box1.max.y),
        f64::max(box0.max.z, box1.max.z));
    AABB { min, max }
}

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self { AABB { min, max } }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        //loop unroll
        {
            let inv_d = 1.0 / ray.d.x;
            let t0 = (self.min.x - ray.o.x) * inv_d;
            let t1 = (self.max.x - ray.o.x) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false
            }
        }
       {
            let inv_d = 1.0 / ray.d.y;
            let t0 = (self.min.y - ray.o.y) * inv_d;
            let t1 = (self.max.y - ray.o.y) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false
            }
        }
       {
            let inv_d = 1.0 / ray.d.z;
            let t0 = (self.min.z - ray.o.z) * inv_d;
            let t1 = (self.max.z - ray.o.z) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false
            }
        }
        true
    }
}
