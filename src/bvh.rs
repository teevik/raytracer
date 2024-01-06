use std::mem::swap;

use vek::{Ray, Vec3};

use crate::interval::Interval;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub axes: Vec3<Interval>,
}

impl Aabb {
    pub fn from_extremes(a: Vec3<f32>, b: Vec3<f32>) -> Self {
        let x = Interval::new(f32::min(a.x, b.x), f32::max(a.x, b.x));
        let y = Interval::new(f32::min(a.y, b.y), f32::max(a.y, b.y));
        let z = Interval::new(f32::min(a.z, b.z), f32::max(a.z, b.z));

        Self {
            axes: Vec3::new(x, y, z),
        }
    }

    pub fn combine(a: Self, b: Self) -> Self {
        let x = Interval::combine(a.axes.x, b.axes.x);
        let y = Interval::combine(a.axes.y, b.axes.y);
        let z = Interval::combine(a.axes.z, b.axes.z);

        Self {
            axes: Vec3::new(x, y, z),
        }
    }

    pub fn raycast(self, ray: Ray<f32>, interval: Interval) -> bool {
        let mut interval = interval;

        for axis in 0..3 {
            let inverse_direction = 1. / ray.direction[axis];
            let origin = ray.origin[axis];

            let mut t0 = (self.axes[axis].min - origin) * inverse_direction;
            let mut t1 = (self.axes[axis].max - origin) * inverse_direction;

            if inverse_direction < 0. {
                swap(&mut t0, &mut t1);
            }

            interval.min = f32::max(t0, interval.min);
            interval.max = f32::min(t1, interval.max);

            if interval.max <= interval.min {
                return false;
            }
        }

        true
    }
}

struct BvhNode<T> {
    value: T,
    bounding_box: Aabb,

    left: Option<Box<BvhNode<T>>>,
    right: Option<Box<BvhNode<T>>>,
}
