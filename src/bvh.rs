use std::mem::swap;

use rand::Rng;
use vek::Vec3;

use crate::data::Ray;
use crate::{
    data::{Hittable, RayHit},
    interval::Interval,
};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    axes: Vec3<Interval>,
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

    pub fn padded(self) -> Self {
        const DELTA: f32 = 0.0001;

        let axes = self.axes.map(|axis| {
            if axis.size() >= DELTA {
                axis
            } else {
                axis.expand(DELTA)
            }
        });

        Self { axes }
    }

    pub fn ray_hits(self, ray: Ray, interval: Interval) -> bool {
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

impl FromIterator<Aabb> for Option<Aabb> {
    fn from_iter<T: IntoIterator<Item = Aabb>>(iter: T) -> Self {
        iter.into_iter()
            .reduce(|acc, bounding_box| Aabb::combine(acc, bounding_box))
    }
}

#[derive(Debug)]
pub enum BvhNode<T> {
    Leaf {
        bounding_box: Aabb,
        left: T,
        right: Option<T>,
    },

    Branch {
        bounding_box: Aabb,
        left: Box<BvhNode<T>>,
        right: Box<BvhNode<T>>,
    },
}

impl<T: Hittable + Clone> BvhNode<T> {
    pub fn new(objects: &[T], rng: &mut impl Rng) -> Option<Self> {
        let axis = rng.gen_range(0..=2);
        let compare_bounding_boxes =
            |a: Aabb, b: Aabb| a.axes[axis].min.partial_cmp(&b.axes[axis].min).unwrap();

        let compare_objects =
            |a: &T, b: &T| compare_bounding_boxes(a.bounding_box(), b.bounding_box());

        match &objects {
            &[] => None,

            &[object] => Some(BvhNode::Leaf {
                bounding_box: object.bounding_box(),
                left: object.clone(),
                right: None,
            }),

            &[left, right] => {
                let [left, right] = if compare_objects(left, right).is_lt() {
                    [left, right]
                } else {
                    [right, left]
                };

                Some(BvhNode::Leaf {
                    bounding_box: Aabb::combine(left.bounding_box(), right.bounding_box()),
                    left: left.clone(),
                    right: Some(right.clone()),
                })
            }

            objects => {
                let mut objects = objects.to_vec();
                objects.sort_unstable_by(compare_objects);

                let middle = objects.len() / 2;

                let (left_objects, right_objects) = objects.split_at(middle);

                let left = Box::new(BvhNode::new(left_objects, rng).unwrap());
                let right = Box::new(BvhNode::new(right_objects, rng).unwrap());

                Some(BvhNode::Branch {
                    bounding_box: Aabb::combine(left.bounding_box(), right.bounding_box()),
                    left,
                    right,
                })
            }
        }
    }
}

impl<T: Hittable> Hittable for BvhNode<T> {
    fn bounding_box(&self) -> Aabb {
        match self {
            BvhNode::Leaf { bounding_box, .. } => *bounding_box,
            BvhNode::Branch { bounding_box, .. } => *bounding_box,
        }
    }

    fn raycast(&self, ray: Ray, interval: Interval) -> Option<RayHit> {
        match self {
            BvhNode::Leaf {
                bounding_box,
                left,
                right,
            } => {
                if !bounding_box.ray_hits(ray, interval) {
                    return None;
                }

                let hit_left = left.raycast(ray, interval);

                let interval = if let Some(ray_hit) = &hit_left {
                    Interval::new(interval.min, ray_hit.distance)
                } else {
                    interval
                };

                let hit_right = right
                    .as_ref()
                    .and_then(|right| right.raycast(ray, interval));

                // Prioritize hit_right since it will always be closer than hit_left
                if hit_right.is_some() {
                    hit_right
                } else {
                    hit_left
                }
            }

            BvhNode::Branch {
                bounding_box,
                left,
                right,
            } => {
                if !bounding_box.ray_hits(ray, interval) {
                    return None;
                }

                let hit_left = left.raycast(ray, interval);

                let interval = if let Some(ray_hit) = &hit_left {
                    Interval::new(interval.min, ray_hit.distance)
                } else {
                    interval
                };

                let hit_right = right.raycast(ray, interval);

                // Prioritize hit_right since it will always be closer than hit_left
                if hit_right.is_some() {
                    hit_right
                } else {
                    hit_left
                }
            }
        }
    }
}
