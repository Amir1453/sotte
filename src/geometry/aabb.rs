use core::f64;

use crate::{
    geometry::{ComputeIntersection, Intersection},
    material::MaterialIndex,
    math::{Ray, Vector},
};

#[derive(Debug)]
pub struct Aabb {
    bounds: crate::larp::BoundingBox,

    material_index: MaterialIndex,
}

impl Aabb {
    #[inline(always)]
    #[must_use]
    pub fn new(
        min: impl Into<Vector>,
        max: impl Into<Vector>,
        material_index: MaterialIndex,
    ) -> super::Object {
        super::Object::Aabb(Aabb {
            bounds: crate::larp::BoundingBox::new(min.into(), max.into()),
            material_index,
        })
    }
}

impl ComputeIntersection for Aabb {
    type Index = MaterialIndex;

    fn intersect(&self, ray: &Ray) -> Option<Intersection<Self::Index>> {
        let bbox = &self.bounds;

        let mut t_enter = f64::NEG_INFINITY;
        let mut t_exit = f64::INFINITY;

        let mut enter_axis = 0usize;
        let mut exit_axis = 0usize;
        let mut enter_normal_sign = 0.0;
        let mut exit_normal_sign = 0.0;

        for i in 0..3 {
            let origin = ray.origin[i];
            let dir = ray.direction[i];

            if dir.abs() < f64::EPSILON {
                if origin < bbox.min[i] || origin > bbox.max[i] {
                    return None;
                }
                continue;
            }

            let inv = 1.0 / dir;
            let mut t0 = (bbox.min[i] - origin) * inv;
            let mut t1 = (bbox.max[i] - origin) * inv;

            let mut near_normal_sign = -1.0;
            let mut far_normal_sign = 1.0;

            if t0 > t1 {
                core::mem::swap(&mut t0, &mut t1);
                near_normal_sign = 1.0;
                far_normal_sign = -1.0;
            }

            if t0 > t_enter {
                t_enter = t0;
                enter_axis = i;
                enter_normal_sign = near_normal_sign;
            }

            if t1 < t_exit {
                t_exit = t1;
                exit_axis = i;
                exit_normal_sign = far_normal_sign;
            }

            if t_exit < t_enter || t_exit < f64::EPSILON {
                return None;
            }
        }

        let (t, hit_axis, normal_sign) = if t_enter >= f64::EPSILON {
            (t_enter, enter_axis, enter_normal_sign)
        } else if t_exit >= f64::EPSILON {
            (t_exit, exit_axis, exit_normal_sign)
        } else {
            return None;
        };

        if !t.is_finite() {
            return None;
        }

        let intersection = ray.at(t);

        let mut normal = Vector::ZERO;
        normal[hit_axis] = normal_sign;

        let uv_coord = |axis: usize| {
            let span = bbox.max[axis] - bbox.min[axis];
            if span.abs() < f64::EPSILON {
                0.0
            } else {
                ((intersection[axis] - bbox.min[axis]) / span).clamp(0.0, 1.0)
            }
        };

        let uv = match hit_axis {
            0 => (uv_coord(2), uv_coord(1)),
            1 => (uv_coord(0), uv_coord(2)),
            2 => (uv_coord(0), uv_coord(1)),
            _ => unreachable!(),
        };

        Some(Intersection::new(
            intersection,
            t,
            normal,
            uv,
            self.material_index,
        ))
    }

    fn shadow_intersect(&self, ray: &Ray) -> Option<f64> {
        let bbox = &self.bounds;
        let mut t_min = f64::EPSILON;
        let mut t_max = f64::INFINITY;

        for i in 0..3 {
            let origin = ray.origin[i];
            let dir = ray.direction[i];

            let (t0, t1) = if dir.abs() < f64::EPSILON {
                if origin < bbox.min[i] || origin > bbox.max[i] {
                    return None;
                }
                (f64::NEG_INFINITY, f64::INFINITY)
            } else {
                let inv = 1.0 / dir;
                let mut t0 = (bbox.min[i] - origin) * inv;
                let mut t1 = (bbox.max[i] - origin) * inv;
                if t0 > t1 {
                    core::mem::swap(&mut t0, &mut t1);
                }
                (t0, t1)
            };

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max < t_min {
                return None;
            }
        }

        Some(t_max)
    }
}

impl crate::larp::Boundable for Aabb {
    fn bounding_box(&self) -> crate::larp::BoundingBox {
        self.bounds.clone()
    }
}

#[derive(Default)]
pub struct AabbBuilder {
    bounds: crate::larp::BoundingBox,

    material_index: MaterialIndex,
}

impl AabbBuilder {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    #[inline]
    #[must_use]
    pub const fn build(self) -> super::Object {
        super::Object::Aabb(Aabb {
            bounds: self.bounds,
            material_index: self.material_index,
        })
    }

    #[inline]
    #[must_use]
    pub fn min(mut self, x: f64, y: f64, z: f64) -> Self {
        self.bounds.min.x = x;
        self.bounds.min.y = y;
        self.bounds.min.z = z;
        self
    }

    #[inline]
    #[must_use]
    pub fn max(mut self, x: f64, y: f64, z: f64) -> Self {
        self.bounds.max.x = x;
        self.bounds.max.y = y;
        self.bounds.max.z = z;
        self
    }

    #[inline]
    #[must_use]
    pub const fn material(mut self, index: MaterialIndex) -> Self {
        self.material_index = index;
        self
    }
}
