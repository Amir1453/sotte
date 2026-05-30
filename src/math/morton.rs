use core::f64;
use std::{cmp::Ordering, marker::PhantomData, ops::*};

use crate::{
    larp::BoundingBox,
    math::{Vector, radix::RadixSorter},
};

pub trait MortonParameter
where
    Self: Copy,
    Self: Sized,
    Self: Send + Sync,
    Self: PartialEq + Eq + PartialOrd + Ord,
    Self: Add<Output = Self>,
    Self: Sub<Output = Self>,
    Self: Mul<Output = Self>,
    Self: Div<Output = Self>,
    Self: Rem<Output = Self>,
    Self: AddAssign,
    Self: SubAssign,
    Self: MulAssign,
    Self: DivAssign,
    Self: RemAssign,
    Self: BitAnd<Output = Self>,
    Self: BitOr<Output = Self>,
    Self: BitXor<Output = Self>,
    Self: Not<Output = Self>,
    Self: BitAndAssign,
    Self: BitOrAssign,
    Self: BitXorAssign,
    Self: Shl<usize, Output = Self>,
    Self: Shr<usize, Output = Self>,
    Self: ShlAssign<usize>,
    Self: ShrAssign<usize>,
{
    const BITS_PER_AXIS: usize;
    const MAX_COORD: f64;
    const BUCKET_SIZE: usize;

    fn voodoo(v: Self) -> Self;
    fn from_f64(v: f64) -> Self;
    fn into_usize(v: Self) -> usize;
}

impl MortonParameter for u32 {
    const BITS_PER_AXIS: usize = 10;
    const MAX_COORD: f64 = ((1 << Self::BITS_PER_AXIS) - 1) as f64;
    const BUCKET_SIZE: usize = 6;

    #[inline]
    fn voodoo(mut v: u32) -> u32 {
        v &= 0x0000_03ff;

        v = (v | (v << 16)) & 0x0300_00ff;
        v = (v | (v << 8)) & 0x0300_f00f;
        v = (v | (v << 4)) & 0x030c_30c3;
        v = (v | (v << 2)) & 0x0924_9249;

        v
    }

    #[inline]
    fn from_f64(value: f64) -> Self {
        value as u32
    }

    #[inline]
    fn into_usize(v: Self) -> usize {
        v as usize
    }
}

impl MortonParameter for u64 {
    const BITS_PER_AXIS: usize = 21;
    const MAX_COORD: f64 = ((1 << Self::BITS_PER_AXIS) - 1) as f64;
    const BUCKET_SIZE: usize = 9;

    #[inline]
    fn voodoo(mut v: u64) -> u64 {
        v &= 0x1fffff;

        v = (v | (v << 32)) & 0x01f00000000ffff;
        v = (v | (v << 16)) & 0x01f0000ff0000ff;
        v = (v | (v << 8)) & 0x100f00f00f00f00f;
        v = (v | (v << 4)) & 0x10c30c30c30c30c3;
        v = (v | (v << 2)) & 0x1249249249249249;

        v
    }

    #[inline]
    fn from_f64(value: f64) -> Self {
        value as u64
    }

    #[inline]
    fn into_usize(v: Self) -> usize {
        v as usize
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash)]
pub struct MortonCode<T: MortonParameter>(pub T);

impl<T: MortonParameter> AsRef<T> for MortonCode<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: MortonParameter> PartialEq<T> for MortonCode<T> {
    fn eq(&self, other: &T) -> bool {
        &self.0 == other
    }
}

impl<T: MortonParameter> PartialOrd<T> for MortonCode<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: MortonParameter> RadixSorter for Vec<MortonCode<T>> {
    fn radix_sort(&mut self) {
        super::radix::radix_sort_by_key::<_, _>(self, 3 * T::BITS_PER_AXIS, |code| {
            T::into_usize(code.0)
        });
    }

    fn par_radix_sort(&mut self) {
        super::radix::par_radix_sort_by_key::<_, _>(self, 3 * T::BITS_PER_AXIS, |code| {
            T::into_usize(code.0)
        });
    }
}

#[derive(Debug, Clone)]
pub struct MortonEncoder<T: MortonParameter> {
    min: Vector,
    inv_extent: Vector,
    phantom: PhantomData<T>,
}

impl<T: MortonParameter> MortonEncoder<T> {
    #[inline(always)]
    #[must_use]
    pub fn new(bounds: &BoundingBox) -> Self {
        let extent = bounds.diagonal();

        Self {
            min: bounds.min.clone(),
            inv_extent: extent.recip(),
            phantom: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub fn encode(&self, p: &Vector) -> MortonCode<T> {
        let n = ((p - &self.min) * &self.inv_extent).clamp_scalar(0.0, 1.);
        let v = n.map(|elem| (elem * T::MAX_COORD).round().clamp(0., T::MAX_COORD));

        Self::morton(T::from_f64(v.x), T::from_f64(v.y), T::from_f64(v.z))
    }

    #[inline]
    #[must_use]
    fn morton(x: T, y: T, z: T) -> MortonCode<T> {
        let code = T::voodoo(x) | (T::voodoo(y) << 1) | (T::voodoo(z) << 2);
        MortonCode(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BBOX: BoundingBox = BoundingBox::new(Vector::ZERO, Vector::splat(10.));

    // ============ u32 Tests ============

    #[test]
    fn test_encode_u32_origin() {
        let encoder = MortonEncoder::<u32>::new(&BBOX);
        let origin = Vector::ZERO;

        let code = encoder.encode(&origin);
        assert_eq!(code.0, 0, "Origin should encode to 0");
    }

    #[test]
    fn test_encode_u32_max_corner() {
        let encoder = MortonEncoder::<u32>::new(&BBOX);
        let max_point = Vector::splat(10.);

        let code = encoder.encode(&max_point);
        let expected_max = (1 << (3 * <u32>::BITS_PER_AXIS)) - 1;
        assert_eq!(
            code, expected_max as u32,
            "Max corner should encode to max value"
        );
    }

    #[test]
    fn test_encode_u32_center() {
        let encoder = MortonEncoder::<u32>::new(&BBOX);
        let center = Vector::splat(5.);

        let code = encoder.encode(&center);
        let max_code = (1 << (3 * <u32>::BITS_PER_AXIS)) - 1;
        assert!(
            code.0 > 0 && code.0 < max_code,
            "Center should encode to middle range"
        );
    }

    #[test]
    fn test_encode_u32_single_axis() {
        let encoder = MortonEncoder::<u32>::new(&BBOX);

        let p1 = Vector::X * 5.;
        let p2 = Vector::Y * 5.;
        let p3 = Vector::Z * 5.;

        let c1 = encoder.encode(&p1);
        let c2 = encoder.encode(&p2);
        let c3 = encoder.encode(&p3);

        assert!(
            c1 != c2 && c2 != c3 && c1 != c3,
            "Different axes should produce different codes"
        );
    }

    #[test]
    fn test_encode_u32_deterministic() {
        let encoder = MortonEncoder::<u32>::new(&BBOX);
        let point = Vector::new(3.7, 2.5, 8.1);

        let code1 = encoder.encode(&point);
        let code2 = encoder.encode(&point);

        assert_eq!(
            code1, code2.0,
            "Same input should always produce same output"
        );
    }

    #[test]
    fn test_encode_u32_in_range() {
        let encoder = MortonEncoder::<u32>::new(&BBOX);

        for i in 0..10 {
            let x = i as f64;
            let y = (9 - i) as f64;
            let point = Vector::new(x, y, 5.0);

            let code = encoder.encode(&point);
            let max_code = (1u64 << (3 * <u32>::BITS_PER_AXIS)) - 1;
            assert!(
                code.0 as u64 <= max_code,
                "Code should fit in allocated bits"
            );
        }
    }

    // ============ u64 Tests ============

    #[test]
    fn test_encode_u64_origin() {
        let encoder = MortonEncoder::<u64>::new(&BBOX);
        let origin = Vector::ZERO;

        let code = encoder.encode(&origin);
        assert_eq!(code, 0, "Origin should encode to 0");
    }

    #[test]
    fn test_encode_u64_max_corner() {
        let encoder = MortonEncoder::<u64>::new(&BBOX);
        let max_point = Vector::splat(10.);

        let code = encoder.encode(&max_point);
        let expected_max = (1u128 << (3 * <u64>::BITS_PER_AXIS)) - 1;
        assert_eq!(
            code, expected_max as u64,
            "Max corner should encode to max value"
        );
    }

    #[test]
    fn test_encode_u64_center() {
        let encoder = MortonEncoder::<u64>::new(&BBOX);
        let center = Vector::splat(5.);

        let code = encoder.encode(&center);
        let max_code = (1u128 << (3 * <u64>::BITS_PER_AXIS)) - 1;
        assert!(
            code > 0 && code < (max_code as u64),
            "Center should encode to middle range"
        );
    }

    #[test]
    fn test_encode_u64_deterministic() {
        let encoder = MortonEncoder::<u64>::new(&BBOX);
        let point = Vector::new(3.7, 2.5, 8.1);

        let code1 = encoder.encode(&point);
        let code2 = encoder.encode(&point);

        assert_eq!(
            code1, code2.0,
            "Same input should always produce same output"
        );
    }

    // ============ Edge Cases ============

    #[test]
    fn test_non_origin_bounds() {
        let min = Vector::splat(5.);
        let max = Vector::splat(15.);

        let bounds = BoundingBox::new(min.clone(), max.clone());
        let encoder = MortonEncoder::<u32>::new(&bounds);

        let code_min = encoder.encode(&min);
        assert_eq!(code_min, 0, "Min corner should encode to 0");

        let code_max = encoder.encode(&max);
        let expected_max = (1 << (3 * <u32>::BITS_PER_AXIS)) - 1;
        assert_eq!(
            code_max, expected_max as u32,
            "Max corner should encode to max"
        );
    }

    #[test]
    fn test_asymmetric_bounds() {
        let bounds = BoundingBox::new(Vector::ZERO, Vector::new(20., 10., 5.));
        let encoder = MortonEncoder::<u32>::new(&bounds);

        let point = Vector::new(10.0, 5.0, 2.5);
        let code = encoder.encode(&point);

        let max_code = (1u64 << (3 * <u32>::BITS_PER_AXIS)) - 1;
        assert!(code.0 as u64 <= max_code);
    }

    #[test]
    fn test_spatial_locality() {
        let encoder = MortonEncoder::<u32>::new(&BBOX);

        let p1 = Vector::splat(5.0);
        let p2 = Vector::new(5.1, 5.0, 5.0);

        let c1 = encoder.encode(&p1);
        let c2 = encoder.encode(&p2);

        let xor = c1.0 ^ c2.0;
        let hamming_distance = xor.count_ones();

        println!("Hamming distance: {}", hamming_distance);
        assert!(
            hamming_distance <= 20,
            "Nearby points should have small Hamming distance"
        );
    }

    // ============ Radix Sort ============

    #[test]
    fn test_radix_sort_encoded_points() {
        let encoder = MortonEncoder::<u32>::new(&BBOX);

        let points = [
            Vector::new(8.0, 1.0, 3.0),
            Vector::new(2.0, 7.0, 1.0),
            Vector::new(5.0, 5.0, 5.0),
            Vector::new(1.0, 1.0, 1.0),
            Vector::new(9.0, 9.0, 9.0),
        ];

        let codes: Vec<_> = points.iter().map(|p| encoder.encode(p)).collect();

        let mut radix = codes.clone();
        radix.radix_sort();

        let mut expected = codes;
        expected.sort();

        assert_eq!(radix, expected);
    }

    #[test]
    fn test_radix_sort_random_u32() {
        fastrand::seed(42);

        let max = 1u32 << (3 * <u32 as MortonParameter>::BITS_PER_AXIS);
        let data: Vec<MortonCode<u32>> = (0..100_000)
            .map(|_| MortonCode(fastrand::u32(..max)))
            .collect();

        let mut radix = data.clone();
        radix.radix_sort();

        let mut expected = data;
        expected.sort();

        assert_eq!(radix, expected);
    }

    #[test]
    fn test_radix_sort_random_u64() {
        fastrand::seed(42);

        let max = 1u64 << (3 * <u64 as MortonParameter>::BITS_PER_AXIS);
        let data: Vec<MortonCode<u64>> = (0..100_000)
            .map(|_| MortonCode(fastrand::u64(..max)))
            .collect();

        let mut radix = data.clone();
        radix.radix_sort();

        let mut expected = data;
        expected.sort();

        assert_eq!(radix, expected);
    }
}
