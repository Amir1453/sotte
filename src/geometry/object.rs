use crate::geometry::{Aabb, Quad, Sphere, TriangleMesh};
use crate::geometry::{ComputeIntersection, Intersection};
use crate::material::MaterialIndex;
use crate::math::{Ray, SurfaceSampleable, Vector};

#[derive(Debug)]
pub enum Object {
    Aabb(Aabb),
    Quad(Quad),
    Sphere(Sphere),
    TriangleMesh(TriangleMesh),
}

impl Object {}

impl ComputeIntersection for Object {
    type Index = MaterialIndex;

    fn intersect(&self, ray: &Ray) -> Option<Intersection<Self::Index>> {
        match self {
            Object::Aabb(bbox) => bbox.intersect(ray),
            Object::Quad(quad) => quad.intersect(ray),
            Object::Sphere(sphere) => sphere.intersect(ray),
            Object::TriangleMesh(mesh) => mesh.intersect(ray),
        }
    }

    fn shadow_intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Object::Aabb(bbox) => bbox.shadow_intersect(ray),
            Object::Quad(quad) => quad.shadow_intersect(ray),
            Object::Sphere(sphere) => sphere.shadow_intersect(ray),
            Object::TriangleMesh(mesh) => mesh.shadow_intersect(ray),
        }
    }
}

impl SurfaceSampleable for Object {
    fn sample(&self) -> (Vector, Vector) {
        match self {
            Object::Aabb(_) => todo!(),
            Object::Quad(quad) => quad.sample(),
            Object::Sphere(sphere) => sphere.sample(),
            Object::TriangleMesh(_) => todo!(),
        }
    }
}
