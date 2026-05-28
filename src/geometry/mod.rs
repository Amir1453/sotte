mod intersection;
mod mesh;
mod object;
mod quad;
mod sphere;

pub use intersection::{ComputeIntersection, Intersection};

pub use object::Object;

use mesh::TriangleMesh;
use quad::Quad;
use sphere::Sphere;

pub use mesh::TriangleMeshBuilder;
pub use quad::QuadBuilder;
pub use sphere::SphereBuilder;
