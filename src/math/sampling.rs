/// TODO: Move to proba module
use crate::math::Vector;

pub trait SurfaceSampleable {
    fn sample(&self) -> (Vector, Vector);
}

pub trait VolumeSampleable {
    fn volume_sample(&self) -> Vector;
}
