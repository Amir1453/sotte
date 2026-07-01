use crate::math::{Ray, Vector};

#[derive(Debug, Clone, Copy)]
pub struct AffineTransform {
    pub first: [f64; 4],
    pub second: [f64; 4],
    pub third: [f64; 4],
}

impl AffineTransform {
    pub const IDENTITY: Self = Self {
        first: [1.0, 0.0, 0.0, 0.0],
        second: [0.0, 1.0, 0.0, 0.0],
        third: [0.0, 0.0, 1.0, 0.0],
    };

    #[inline(always)]
    #[must_use]
    pub const fn new(first: [f64; 4], second: [f64; 4], third: [f64; 4]) -> Self {
        Self {
            first,
            second,
            third,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn apply_point(&self, point: &Vector) -> Vector {
        Vector::new(
            self.first[0] * point.x
                + self.first[1] * point.y
                + self.first[2] * point.z
                + self.first[3],
            self.second[0] * point.x
                + self.second[1] * point.y
                + self.second[2] * point.z
                + self.second[3],
            self.third[0] * point.x
                + self.third[1] * point.y
                + self.third[2] * point.z
                + self.third[3],
        )
    }

    #[inline(always)]
    #[must_use]
    pub fn apply_vector(&self, vector: &Vector) -> Vector {
        Vector::new(
            self.first[0] * vector.x + self.first[1] * vector.y + self.first[2] * vector.z,
            self.second[0] * vector.x + self.second[1] * vector.y + self.second[2] * vector.z,
            self.third[0] * vector.x + self.third[1] * vector.y + self.third[2] * vector.z,
        )
    }

    #[inline(always)]
    #[must_use]
    pub fn apply_ray(&self, ray: &Ray) -> Ray {
        Ray::new(
            self.apply_point(&ray.origin),
            self.apply_vector(&ray.direction),
        )
    }

    #[inline]
    #[must_use]
    pub fn translate(offset: Vector) -> Self {
        Self {
            first: [1.0, 0.0, 0.0, offset.x],
            second: [0.0, 1.0, 0.0, offset.y],
            third: [0.0, 0.0, 1.0, offset.z],
        }
    }

    #[inline]
    #[must_use]
    pub fn scale(scale: Vector) -> Self {
        Self {
            first: [scale.x, 0.0, 0.0, 0.0],
            second: [0.0, scale.y, 0.0, 0.0],
            third: [0.0, 0.0, scale.z, 0.0],
        }
    }

    #[inline]
    #[must_use]
    pub fn rotate_y(theta: f64) -> Self {
        let sin = theta.sin();
        let cos = theta.cos();

        Self {
            first: [cos, 0.0, sin, 0.0],
            second: [0.0, 1.0, 0.0, 0.0],
            third: [-sin, 0.0, cos, 0.0],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub object_to_world: AffineTransform,
    pub world_to_object: AffineTransform,
}

impl Transform {
    #[inline(always)]
    #[must_use]
    pub const fn new(object_to_world: AffineTransform, world_to_object: AffineTransform) -> Self {
        Self {
            object_to_world,
            world_to_object,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            object_to_world: AffineTransform::IDENTITY,
            world_to_object: AffineTransform::IDENTITY,
        }
    }

    #[inline]
    #[must_use]
    pub fn translate(offset: Vector) -> Self {
        Self {
            object_to_world: AffineTransform::translate(offset.clone()),
            world_to_object: AffineTransform::translate(-offset),
        }
    }

    #[inline]
    #[must_use]
    pub fn scale(scale: Vector) -> Self {
        Self {
            object_to_world: AffineTransform::scale(scale.clone()),
            world_to_object: AffineTransform::scale(scale.recip()),
        }
    }

    #[inline]
    #[must_use]
    pub fn rotate_y(theta: f64) -> Self {
        Self {
            object_to_world: AffineTransform::rotate_y(theta),
            world_to_object: AffineTransform::rotate_y(-theta),
        }
    }
}
