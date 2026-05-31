use crate::{
    larp::{Boundable, BoundingBox},
    math::{MortonEncoder, MortonParameter, RadixSorter, Ray},
};

#[derive(Debug, Clone)]
enum LinearBvhNode {
    Internal {
        bbox: BoundingBox,
        left: u32,
        right: u32,
    },

    Leaf {
        bbox: BoundingBox,
        primitive_index: u32,
    },
}

impl LinearBvhNode {
    pub const EMPTY_LEAF: Self = Self::Leaf {
        primitive_index: 0,
        bbox: BoundingBox::EMPTY,
    };

    #[inline(always)]
    #[must_use]
    pub const fn new_internal(bbox: BoundingBox, left: usize, right: usize) -> Self {
        Self::Internal {
            bbox,
            left: left as u32,
            right: right as u32,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn new_leaf(bbox: BoundingBox, primitive_index: usize) -> Self {
        Self::Leaf {
            bbox,
            primitive_index: primitive_index as u32,
        }
    }
}

impl Boundable for LinearBvhNode {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            Self::Internal { bbox, .. } => bbox.clone(),
            Self::Leaf { bbox, .. } => bbox.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MortonPrimitive {
    morton_code: u32,
    primitive_index: usize,
}

impl PartialEq for MortonPrimitive {
    fn eq(&self, other: &Self) -> bool {
        self.morton_code == other.morton_code
    }
}

impl Eq for MortonPrimitive {}

impl PartialOrd for MortonPrimitive {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MortonPrimitive {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.morton_code.cmp(&other.morton_code)
    }
}

impl RadixSorter for Vec<MortonPrimitive> {
    fn radix_sort(&mut self) {
        crate::math::radix_sort_by_key::<_, _>(self, 3 * <u32>::BITS_PER_AXIS, |code| {
            code.morton_code as usize
        });
    }

    fn par_radix_sort(&mut self) {
        crate::math::par_radix_sort_by_key::<_, _>(self, 3 * <u32>::BITS_PER_AXIS, |code| {
            code.morton_code as usize
        });
    }
}

#[derive(Debug, Clone)]
pub struct LinearBvh {
    nodes: Vec<LinearBvhNode>,
}

impl LinearBvh {
    #[inline(always)]
    #[must_use]
    pub const fn empty() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn build_seq<T: Boundable>(primitives: &[T]) -> Self {
        if primitives.is_empty() {
            return Self::empty();
        }

        let primitive_count = primitives.len();
        let encoder = MortonEncoder::<u32>::new(&primitives.bounding_box());

        let cached_bboxes: Vec<BoundingBox> =
            primitives.iter().map(Boundable::bounding_box).collect();

        let mut morton_primitives: Vec<MortonPrimitive> = cached_bboxes
            .iter()
            .enumerate()
            .map(|(primitive_index, bbox)| MortonPrimitive {
                morton_code: encoder.encode(&bbox.center()).0,
                primitive_index,
            })
            .collect();

        morton_primitives.radix_sort();

        let mut nodes = vec![LinearBvhNode::EMPTY_LEAF; 2 * primitive_count - 1];

        let leaf_offset = primitive_count - 1;
        for leaf_idx in 0..primitive_count {
            let node_idx = leaf_offset + leaf_idx;

            let prim_idx = morton_primitives[leaf_idx].primitive_index as usize;
            nodes[node_idx] = LinearBvhNode::new_leaf(cached_bboxes[prim_idx].clone(), prim_idx);
        }

        // Build internal topology.
        for i in 0..(primitive_count - 1) {
            let (first, split, last) = Self::find_range_and_split(&morton_primitives, i);

            let left = if first == split {
                leaf_offset + split
            } else {
                split
            };

            let right = if split + 1 == last {
                leaf_offset + split + 1
            } else {
                split + 1
            };

            nodes[i] = LinearBvhNode::new_internal(BoundingBox::EMPTY, left, right);
        }

        let _ = Self::propagate_bboxes(&mut nodes, 0);

        LinearBvh { nodes }
    }

    pub fn build_par<T: Boundable + Sync>(primitives: &[T]) -> Self {
        use rayon::prelude::*;

        if primitives.is_empty() {
            return Self::empty();
        }

        let primitive_count = primitives.len();
        let encoder = MortonEncoder::<u32>::new(&primitives.bounding_box());

        let cached_bboxes: Vec<BoundingBox> =
            primitives.iter().map(Boundable::bounding_box).collect();

        let mut morton_primitives: Vec<MortonPrimitive> = cached_bboxes
            .par_iter()
            .enumerate()
            .map(|(primitive_index, bbox)| MortonPrimitive {
                morton_code: encoder.encode(&bbox.center()).0,
                primitive_index,
            })
            .collect();

        morton_primitives.par_sort();

        let leaf_offset = primitive_count - 1;
        let mut nodes = vec![LinearBvhNode::EMPTY_LEAF; 2 * primitive_count - 1];

        nodes[leaf_offset..]
            .par_iter_mut()
            .enumerate()
            .for_each(|(leaf_index, node)| {
                let prim_idx = morton_primitives[leaf_index].primitive_index as usize;

                *node = LinearBvhNode::new_leaf(cached_bboxes[prim_idx].clone(), prim_idx);
            });

        nodes[..primitive_count - 1]
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, node)| {
                let (first, split, last) = Self::find_range_and_split(&morton_primitives, i);

                let left = if first == split {
                    leaf_offset + split
                } else {
                    split
                };

                let right = if split + 1 == last {
                    leaf_offset + split + 1
                } else {
                    split + 1
                };

                *node = LinearBvhNode::new_internal(BoundingBox::EMPTY, left, right);
            });

        let _ = Self::propagate_bboxes(&mut nodes, 0);

        Self { nodes }
    }

    #[inline]
    fn delta(morton_primitives: &[MortonPrimitive], i: isize, j: isize) -> i32 {
        if j < 0 || j >= morton_primitives.len() as isize {
            return -1;
        }

        let a = ((morton_primitives[i as usize].morton_code as u64) << 32)
            | (morton_primitives[i as usize].primitive_index as u64);

        let b = ((morton_primitives[j as usize].morton_code as u64) << 32)
            | (morton_primitives[j as usize].primitive_index as u64);

        (a ^ b).leading_zeros() as i32
    }

    #[inline]
    fn find_range_and_split(
        morton_primitives: &[MortonPrimitive],
        i: usize,
    ) -> (usize, usize, usize) {
        let i = i as isize;

        let delta_left = Self::delta(morton_primitives, i, i - 1);
        let delta_right = Self::delta(morton_primitives, i, i + 1);

        let d: isize = if delta_right > delta_left { 1 } else { -1 };
        let delta_min = Self::delta(morton_primitives, i, i - d);

        let mut l_max: isize = 2;
        while Self::delta(morton_primitives, i, i + l_max * d) > delta_min {
            l_max *= 2;
        }

        let mut l: isize = 0;
        let mut step = l_max / 2;
        while step > 0 {
            if Self::delta(morton_primitives, i, i + (l + step) * d) > delta_min {
                l += step;
            }
            step /= 2;
        }

        let j = i + l * d;
        let first = i.min(j) as usize;
        let last = i.max(j) as usize;

        let delta_node = Self::delta(morton_primitives, first as isize, last as isize);

        let mut split = first as isize;
        let mut span = (last - first) as isize;

        while span > 1 {
            span = (span + 1) / 2;
            let new_split = split + span;

            if new_split < last as isize {
                let delta_split = Self::delta(morton_primitives, first as isize, new_split);

                if delta_split > delta_node {
                    split = new_split;
                }
            }
        }

        (first, split as usize, last)
    }

    #[inline]
    #[must_use]
    fn propagate_bboxes(nodes: &mut [LinearBvhNode], node_index: u32) -> BoundingBox {
        match &nodes[node_index as usize] {
            LinearBvhNode::Leaf { bbox, .. } => {
                return bbox.clone();
            }

            LinearBvhNode::Internal { left, right, .. } => {
                let left = *left;
                let right = *right;

                let left_bbox = Self::propagate_bboxes(nodes, left);
                let right_bbox = Self::propagate_bboxes(nodes, right);

                let bbox = left_bbox.union(&right_bbox);

                if let LinearBvhNode::Internal {
                    bbox: node_bbox, ..
                } = &mut nodes[node_index as usize]
                {
                    *node_bbox = bbox.clone();
                }

                bbox
            }
        }
    }

    pub fn intersect_ray(&self, ray: &Ray, t_min: f64, t_max: f64) -> Vec<usize> {
        let mut hits = Vec::with_capacity(16);

        if self.nodes.is_empty() {
            return hits;
        }

        let mut stack = [0usize; 64];
        let mut rsp = 1usize;

        while rsp > 0 {
            rsp -= 1;
            let node_index = stack[rsp];
            let node = &self.nodes[node_index];

            let bbox = match node {
                LinearBvhNode::Internal { bbox, .. } => bbox,
                LinearBvhNode::Leaf { bbox, .. } => bbox,
            };

            if !bbox.is_intersecting(ray, t_min, t_max) {
                continue;
            }

            match node {
                LinearBvhNode::Leaf {
                    primitive_index, ..
                } => {
                    hits.push(*primitive_index as usize);
                }

                LinearBvhNode::Internal { left, right, .. } => {
                    stack[rsp] = *right as usize;
                    rsp += 2;
                    stack[rsp - 1] = *left as usize;
                }
            }
        }

        hits
    }
}

impl Boundable for LinearBvh {
    fn bounding_box(&self) -> BoundingBox {
        self.nodes
            .first()
            .map(Boundable::bounding_box)
            .unwrap_or(BoundingBox::EMPTY)
    }
}
