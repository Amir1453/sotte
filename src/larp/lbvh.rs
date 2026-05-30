#![allow(unused)]

use crate::{
    larp::{Boundable, BoundingBox},
    math::{Vector, morton::MortonEncoder},
};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum LinearBvhNode {
    Internal {
        parent: Option<usize>,
        left: usize,
        right: usize,
        bbox: BoundingBox,
    },
    Leaf {
        parent: Option<usize>,
        primitive_index: usize,
        bbox: BoundingBox,
    },
}

impl LinearBvhNode {
    pub const EMPTY_LEAF: Self = Self::Leaf {
        parent: None,
        primitive_index: 0,
        bbox: BoundingBox::EMPTY,
    };

    #[inline(always)]
    #[must_use]
    pub const fn new_internal(
        parent: Option<usize>,
        bbox: BoundingBox,
        left: usize,
        right: usize,
    ) -> Self {
        Self::Internal {
            parent,
            bbox,
            left,
            right,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn new_leaf(
        parent: Option<usize>,
        bbox: BoundingBox,
        primitive_index: usize,
    ) -> Self {
        Self::Leaf {
            parent,
            bbox,
            primitive_index,
        }
    }

    #[inline]
    #[must_use]
    pub const fn parent(&self) -> Option<usize> {
        match self {
            Self::Internal { parent, .. } | Self::Leaf { parent, .. } => *parent,
        }
    }

    #[inline]
    pub const fn set_parent(&mut self, parent: Option<usize>) {
        match self {
            Self::Internal { parent: p, .. } | Self::Leaf { parent: p, .. } => *p = parent,
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
    bounding_box: BoundingBox,
    morton_code: u32,
}

impl MortonPrimitive {
    #[inline]
    #[must_use]
    fn with_primitive<T: Boundable>(primitive: &T, encoder: &MortonEncoder<u32>) -> Self {
        let bounding_box = primitive.bounding_box();
        let morton_code = encoder.encode(&bounding_box.center());

        MortonPrimitive {
            bounding_box,
            morton_code: morton_code.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinearBvh {
    nodes: Vec<LinearBvhNode>,
    primitive_indices: Vec<usize>,
}

impl LinearBvh {
    #[inline(always)]
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            nodes: Vec::new(),
            primitive_indices: Vec::new(),
        }
    }

    #[inline]
    fn delta(
        morton_primitives: &[MortonPrimitive],
        primitive_indices: &[usize],
        i: isize,
        j: isize,
    ) -> i32 {
        if j < 0 || j >= morton_primitives.len() as isize {
            return -1;
        }

        let i = i as usize;
        let j = j as usize;

        let a = Self::morton_key(morton_primitives, primitive_indices, i);
        let b = Self::morton_key(morton_primitives, primitive_indices, j);

        (a ^ b).leading_zeros() as i32
    }

    #[inline]
    #[must_use]
    fn morton_key(
        morton_primitives: &[MortonPrimitive],
        primitive_indices: &[usize],
        i: usize,
    ) -> u64 {
        ((morton_primitives[i].morton_code as u64) << 32) | (primitive_indices[i] as u64)
    }

    #[inline]
    fn find_range_and_split(
        morton_primitives: &[MortonPrimitive],
        primitive_indices: &[usize],
        i: usize,
    ) -> (usize, usize, usize) {
        let i = i as isize;

        let delta_left = Self::delta(morton_primitives, primitive_indices, i, i - 1);
        let delta_right = Self::delta(morton_primitives, primitive_indices, i, i + 1);

        // Paper uses sign(delta(i,i+1) - delta(i,i-1)).
        // In practice, choose right on strict improvement, otherwise left.
        let d: isize = if delta_right > delta_left { 1 } else { -1 };

        let delta_min = Self::delta(morton_primitives, primitive_indices, i, i - d);

        // Find an upper bound for the range length.
        let mut l_max: isize = 2;
        while Self::delta(morton_primitives, primitive_indices, i, i + l_max * d) > delta_min {
            l_max *= 2;
        }

        // Binary search the actual range length.
        let mut l: isize = 0;
        let mut t = l_max / 2;
        while t >= 1 {
            if Self::delta(morton_primitives, primitive_indices, i, i + (l + t) * d) > delta_min {
                l += t;
            }
            t /= 2;
        }

        let j = i + l * d;

        // Search split position.
        let delta_node = Self::delta(morton_primitives, primitive_indices, i, j);

        let mut s: isize = 0;
        let mut t = (l + 1) / 2;
        while t >= 1 {
            if Self::delta(morton_primitives, primitive_indices, i, i + (s + t) * d) > delta_node {
                s += t;
            }
            t /= 2;
        }

        let gamma = i + s * d + d.min(0);

        let first = i.min(j) as usize;
        let last = i.max(j) as usize;
        let split = gamma as usize;

        (first, split, last)
    }

    #[inline]
    #[must_use]
    fn propagate_bboxes(nodes: &mut [LinearBvhNode], node_index: usize) -> BoundingBox {
        let node = nodes[node_index].clone();

        match node {
            LinearBvhNode::Leaf { bbox, .. } => bbox,

            LinearBvhNode::Internal { left, right, .. } => {
                let left_bbox = Self::propagate_bboxes(nodes, left);
                let right_bbox = Self::propagate_bboxes(nodes, right);
                let bbox = left_bbox.union(&right_bbox);

                if let LinearBvhNode::Internal {
                    bbox: node_bbox, ..
                } = &mut nodes[node_index]
                {
                    *node_bbox = bbox.clone();
                }

                bbox
            }
        }
    }

    pub fn build<T: Boundable>(primitives: &[T]) -> Self {
        if primitives.is_empty() {
            return Self::empty();
        }

        let primitive_count = primitives.len();
        let primitive_indices: Vec<usize> = (0..primitive_count).collect();

        let encoder = MortonEncoder::<u32>::new(&primitives.bounding_box());

        let morton_primitives = primitives
            .iter()
            .map(|primitive| MortonPrimitive::with_primitive(primitive, &encoder))
            .collect::<Vec<MortonPrimitive>>();

        let mut zipped: Vec<(usize, MortonPrimitive)> = primitive_indices
            .into_iter()
            .zip(morton_primitives)
            .collect();

        zipped.sort_unstable_by_key(|p| p.1.morton_code);

        let (primitive_indices, morton_primitives): (Vec<usize>, Vec<MortonPrimitive>) =
            zipped.into_iter().unzip();

        let mut nodes = vec![LinearBvhNode::EMPTY_LEAF; 2 * primitive_count - 1];

        let leaf_offset = primitive_count - 1;
        for leaf_idx in 0..primitive_count {
            let node_idx = leaf_offset + leaf_idx;

            nodes[node_idx] = LinearBvhNode::new_leaf(
                None,
                morton_primitives[leaf_idx].bounding_box.clone(),
                primitive_indices[leaf_idx],
            );
        }

        // Build internal topology.
        if primitive_count > 1 {
            for i in 0..(primitive_count - 1) {
                let (first, split, last) =
                    Self::find_range_and_split(&morton_primitives, &primitive_indices, i);

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

                nodes[left].set_parent(Some(i));
                nodes[right].set_parent(Some(i));

                let parent = nodes[i].parent();
                nodes[i] = LinearBvhNode::new_internal(parent, BoundingBox::EMPTY, left, right);
            }

            let _ = Self::propagate_bboxes(&mut nodes, 0);
        }

        LinearBvh {
            nodes,
            primitive_indices,
        }
    }
}
