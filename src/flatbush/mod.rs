/**
 * A fast, low memory footprint static Rtree.
 *
 * Original implementation in Javascript: https://github.com/mourner/flatbush
 * Initial conversion to rust by Jacob Wasserman @jwass
 */
use num_traits::PrimInt;

use itertools::iproduct;
mod hilbert;

use crate::primitives::{Coordinate, Envelope, HasEnvelope, Position, Rect};
use hilbert::Hilbert;

pub const FLATBUSH_DEFAULT_DEGREE: usize = 8;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Flatbush<C>
where
    C: Coordinate,
{
    pub degree: usize,
    // nodes in level i are (level_indices[i] .. level_indices[i + 1] - 1)
    level_indices: Vec<usize>,
    tree: Vec<(usize, Envelope<C>)>,
}

impl<C: Coordinate> HasEnvelope<C> for Flatbush<C> {
    fn envelope(&self) -> Envelope<C> {
        self.root_node().envelope
    }
}

#[allow(dead_code)]
impl<C> Flatbush<C>
where
    C: Coordinate,
{
    pub fn new_empty() -> Flatbush<C> {
        Flatbush {
            degree: FLATBUSH_DEFAULT_DEGREE,
            level_indices: vec![0],
            tree: vec![(0, Envelope::Empty)],
        }
    }

    pub fn new(items: &Vec<impl HasEnvelope<C>>, degree: usize) -> Flatbush<C> {
        let total_envelope = Envelope::from_envelopes(items.iter().map(|e| e.envelope()));
        let hilbert_square: Hilbert<C>;
        match total_envelope {
            Envelope::Empty => {
                // The list of items are empty, or all items are empty.
                return Flatbush::new_unsorted(items, degree);
            }
            Envelope::Bounds(rect) => hilbert_square = Hilbert::new(rect),
        }

        let mut entries: Vec<(u32, usize, Envelope<C>)> = items
            .iter()
            .map(|e| e.envelope())
            .enumerate()
            .map(|(i, e)| (hilbert_square.safe_hilbert(e.center()), i, e))
            .collect();

        entries.sort_unstable_by_key(|&(h, _, _)| h);

        Flatbush::_new_unsorted(
            entries.into_iter().map(|(_, i, e)| (i, e)).collect(),
            degree,
        )
    }

    pub fn new_unsorted(items: &Vec<impl HasEnvelope<C>>, degree: usize) -> Flatbush<C> {
        let entries = items.iter().map(|e| e.envelope()).enumerate().collect();
        Flatbush::_new_unsorted(entries, degree)
    }

    fn _new_unsorted(entries: Vec<(usize, Envelope<C>)>, degree: usize) -> Flatbush<C> {
        if degree != degree.next_power_of_two() {
            panic!("Degree must be a positive power of 2.");
        }
        let degree_exp = degree.trailing_zeros();

        if entries.len() == 0 {
            return Flatbush::new_empty();
        }

        let mut tree: Vec<(usize, Envelope<C>)> = Vec::with_capacity(3 * entries.len() / 2);
        tree.extend(entries.iter());

        let estimated_capacity = quick_log_ceil(entries.len(), degree_exp) + 1;
        let mut level_indices: Vec<usize> = Vec::with_capacity(estimated_capacity as usize);
        level_indices.push(0);

        let mut level = 0;
        let mut level_size = entries.len();
        let mut level_capacity;

        while level_size > 1 {
            level_capacity = next_multiple(level_size, degree);
            level_indices.push(level_indices[level] + level_capacity);
            // Pad out the remaining spaces with empties that will never match.
            let mut dummy_index = level_size;
            while tree.len() < level_indices[level + 1] {
                tree.push((dummy_index, Envelope::Empty));
                dummy_index += 1;
            }

            let level_items = &tree[level_indices[level]..level_indices[level + 1]];
            let chunks = level_items.chunks(degree);
            let next_items: Vec<Envelope<C>> = chunks
                .map(|items| Envelope::from_envelopes(items.iter().map(|(_, e)| *e)))
                .collect();
            tree.extend(next_items.into_iter().enumerate());

            // Set up variables for the next level.
            level += 1;
            level_size = level_capacity / degree;
        }

        tree.shrink_to_fit();
        level_indices.shrink_to_fit();

        Flatbush {
            degree,
            level_indices,
            tree,
        }
    }

    /**
     * Find geometries that might intersect the query_rect.
     *
     * This only checks bounding-box intersection, so the candidates must be
     * checked by the caller.
     */
    pub fn find_intersection_candidates<E: Into<Envelope<C>>>(&self, query: E) -> Vec<usize> {
        let query_env: Envelope<C> = query.into();
        let mut todo_list: Vec<FlatbushNode<C>> =
            Vec::with_capacity(self.degree * self.level_indices.len());
        let mut results = Vec::new();

        self._maybe_push_isxn(self.root_node(), query_env, &mut results, &mut todo_list);

        // The todo_list will keep a LIFO stack of nodes to be processed.
        // The invariant is that everything in todo_list (envelope) intersects
        // query_rect, and is level > 0 (leaves are yielded).
        while let Some(node) = todo_list.pop() {
            self.get_children(node).iter().for_each(|&child| {
                self._maybe_push_isxn(child, query_env, &mut results, &mut todo_list);
            });
        }

        results
    }

    fn _maybe_push_isxn(
        &self,
        node: FlatbushNode<C>,
        query_env: Envelope<C>,
        results: &mut Vec<usize>,
        todo_list: &mut Vec<FlatbushNode<C>>,
    ) {
        if !node.envelope.intersects(query_env) {
            return;
        }
        if node.level == 0 {
            results.push(node.sibling_index);
        } else {
            todo_list.push(node);
        }
    }

    /**
     * Find geometries that might be within `distance` of `position`.
     *
     * This only checks bounding-box distance, so the candidates must be
     * checked by the caller.
     */
    pub fn find_candidates_within(&self, position: Position<C>, distance: C) -> Vec<usize> {
        let delta = Position::new(distance, distance);
        self.find_intersection_candidates(Rect::new(position - delta, position + delta))
    }

    /**
     * Find all distinct elements of the Rtree that might intersect each other.
     *
     * This will only return each candidate pair once; the element with the
     * smaller index will be the first element of the pair.  It will not return
     * the degenerate pair of two of the same elements.
     *
     * This only checks bounding-box intersection, so the candidates must be
     * checked by the caller.
     */
    pub fn find_self_intersection_candidates(&self) -> Vec<(usize, usize)> {
        let mut results = Vec::new();

        // The todo_list will keep a LIFO stack of pairs of nodes to be processed.
        // The invariants for the todo_list are:
        // * The first node in the pair is from self, the second from other
        // * The nodes in the pair envelope intersect
        // * The nodes in the pair are at the same level
        // * The nodes are level > 0 (leaves are yielded).
        let mut todo_list: Vec<(FlatbushNode<C>, FlatbushNode<C>)> =
            Vec::with_capacity(self.degree * self.level_indices.len());
        let root_node = self.root_node();

        self._maybe_push_self_isxn(root_node, root_node, &mut results, &mut todo_list);

        while let Some((node1, node2)) = todo_list.pop() {
            let children1: Vec<FlatbushNode<C>>;
            let children2: Vec<FlatbushNode<C>>;
            if node1.tree_index == node2.tree_index {
                // They are the same node, so we don't need to do the isxn checks.
                children1 = self.get_children(node1);
                children2 = self.get_children(node2);
            } else {
                children1 = self
                    .get_children(node1)
                    .into_iter()
                    .filter(|c1| c1.envelope.intersects(node2.envelope))
                    .collect();
                children2 = self
                    .get_children(node2)
                    .into_iter()
                    .filter(|c2| c2.envelope.intersects(node1.envelope))
                    .collect();
            }
            iproduct!(children1, children2).for_each(|(c1, c2)| {
                self._maybe_push_self_isxn(c1, c2, &mut results, &mut todo_list)
            });
        }

        results
    }

    fn _maybe_push_self_isxn(
        &self,
        node1: FlatbushNode<C>,
        node2: FlatbushNode<C>,
        results: &mut Vec<(usize, usize)>,
        todo_list: &mut Vec<(FlatbushNode<C>, FlatbushNode<C>)>,
    ) {
        // Dedup results, and check for intersection.
        if node1.tree_index > node2.tree_index || !node1.envelope.intersects(node2.envelope) {
            return;
        }
        match (node1.level, node2.level) {
            (0, 0) => {
                if node1.sibling_index != node2.sibling_index {
                    results.push((
                        node1.sibling_index.min(node2.sibling_index),
                        node1.sibling_index.max(node2.sibling_index),
                    ))
                }
            }
            (0, _) | (_, 0) => {
                panic!("Self-intersection found with different levels.");
            }
            _ => {
                todo_list.push((node1, node2));
            }
        }
    }

    /**
     * Find all pairs of elements of this rtree and the other that might intersect.
     *
     * This will return pairs where the first index is for the element in this
     * rtree, and the second index is for the other rtree.
     *
     * This only checks bounding-box intersection, so the candidates must be
     * checked by the caller.
     */
    pub fn find_other_rtree_intersection_candidates(
        &self,
        other: &Flatbush<C>,
    ) -> Vec<(usize, usize)> {
        let mut results = Vec::new();

        // The todo_list will keep a LIFO stack of pairs of nodes to be processed.
        // The invariants for the todo_list are:
        // * The first node in the pair is from self, the second from other
        // * The nodes in the pair envelope intersect
        // * At least one node is level > 0 (leaves are yielded).
        let mut todo_list: Vec<(FlatbushNode<C>, FlatbushNode<C>)> =
            Vec::with_capacity(self.degree * self.level_indices.len());
        self._maybe_push_other_isxn(self.root_node(), other.root_node(), &mut todo_list);

        while let Some((node1, node2)) = todo_list.pop() {
            if node1.level == 0 && node2.level == 0 {
                results.push((node1.sibling_index, node2.sibling_index));
            } else if node1.level >= node2.level {
                for child1 in self.get_children(node1) {
                    self._maybe_push_other_isxn(child1, node2, &mut todo_list);
                }
            } else {
                // node2.level > node1.level
                for child2 in other.get_children(node2) {
                    self._maybe_push_other_isxn(node1, child2, &mut todo_list);
                }
            }
        }

        results
    }

    fn _maybe_push_other_isxn(
        &self,
        node1: FlatbushNode<C>,
        node2: FlatbushNode<C>,
        todo_list: &mut Vec<(FlatbushNode<C>, FlatbushNode<C>)>,
    ) {
        if !node1.envelope.intersects(node2.envelope) {
            return;
        }
        todo_list.push((node1, node2));
    }

    pub fn root_node(&self) -> FlatbushNode<C> {
        let tree_index = self.tree.len() - 1;
        FlatbushNode {
            level: self.level_indices.len() - 1,
            tree_index: tree_index,
            sibling_index: self.tree[tree_index].0,
            envelope: self.tree[tree_index].1,
        }
    }

    pub fn get_children(&self, node: FlatbushNode<C>) -> Vec<FlatbushNode<C>> {
        let child_level = node.level - 1;
        let start_index = self.level_indices[child_level] + node.sibling_index * self.degree;
        let child_index_range = start_index..start_index + self.degree;
        child_index_range
            .map(move |tree_index| FlatbushNode {
                level: child_level,
                tree_index: tree_index,
                sibling_index: self.tree[tree_index].0,
                envelope: self.tree[tree_index].1,
            })
            .collect()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlatbushNode<C: Coordinate> {
    // Level in tree, 0 is leaf, max is root.
    pub level: usize,
    // The index within the tree
    pub tree_index: usize,
    // Index of node in a level
    pub sibling_index: usize,
    pub envelope: Envelope<C>,
}

/**
 * Take a quick ceil(log(n, 2**e)).
 *
 * n is the number to take the log of.
 * e is the exponent of the base.
 * We define quick_log_ceil(0, e) == 0.
 * quick_log_ceil(n, 0) is illegal and will panic.
 *
 * so quick_log_ceil(15, 3) == ceil(log(15, 8)) == 2
 */
fn quick_log_ceil(n: usize, e: u32) -> u32 {
    if e == 0 {
        panic!("Cannot call quick_log_ceil with exponent 0.");
    }
    if n < 2 {
        return 0;
    }
    let n_pow2 = n.next_power_of_two();
    let n_log_2 = n_pow2.trailing_zeros();
    div_ceil(n_log_2 as u32, e as u32)
}

/**
 * Calculate ceil(n/k) with integer ops.
 */
fn div_ceil<I: PrimInt>(n: I, k: I) -> I {
    let b = if (n % k) != I::zero() {
        I::one()
    } else {
        I::zero()
    };
    n / k + b
}

/**
 * Return least multiple of k that is equal to or greater than n.
 */
fn next_multiple<I: PrimInt>(n: I, k: I) -> I {
    k * div_ceil(n, k)
}

#[cfg(test)]
mod tests {
    use super::{div_ceil, iproduct, next_multiple, quick_log_ceil, Envelope, Flatbush, Rect};

    #[test]
    fn test_quick_log_ciel() {
        assert_eq!(quick_log_ceil(1, 1), 0);
        assert_eq!(quick_log_ceil(7, 3), 1);
        assert_eq!(quick_log_ceil(8, 3), 1);
        assert_eq!(quick_log_ceil(9, 3), 2);
        assert_eq!(quick_log_ceil(8 * 8 + 1, 3), 3);
        assert_eq!(quick_log_ceil(4usize.pow(7), 2), 7);
        assert_eq!(quick_log_ceil(4usize.pow(7) - 1, 2), 7);
        assert_eq!(quick_log_ceil(4usize.pow(7) / 2, 2), 7);
        assert_eq!(quick_log_ceil(4usize.pow(7) + 1, 2), 8);
    }

    #[test]
    fn test_div_ceiling() {
        assert_eq!(div_ceil(1, 1), 1);
        assert_eq!(div_ceil(2, 2), 1);
        assert_eq!(div_ceil(2, 1), 2);
        assert_eq!(div_ceil(1, 2), 1);
    }

    #[test]
    fn test_next_multiple() {
        assert_eq!(next_multiple(2, 2), 2);
        assert_eq!(next_multiple(3, 2), 4);
        assert_eq!(next_multiple(65, 8), 72);
        assert_eq!(next_multiple(0, 8), 0);
    }

    #[test]
    fn test_empty_tree() {
        let empty = Flatbush::new_empty();
        let query_rect = Rect::from(((0., 0.), (1., 1.)));
        assert_eq!(empty.find_intersection_candidates(query_rect), vec![]);
        assert_eq!(empty.find_self_intersection_candidates(), vec![]);
    }

    #[test]
    fn test_build_tree_unsorted() {
        let degree = 4;
        let e0 = Envelope::from(((7.0f32, 44.), (8., 48.)));
        let e1 = Envelope::from(((25., 48.), (35., 55.)));
        let e2 = Envelope::from(((98., 46.), (99., 56.)));
        let e3 = Envelope::from(((58., 65.), (73., 79.)));
        let e4 = Envelope::from(((43., 40.), (44., 45.)));
        let e5 = Envelope::from(((97., 87.), (100., 91.)));
        let e6 = Envelope::from(((92., 46.), (108., 57.)));
        let e7 = Envelope::from(((7.1, 48.), (10., 56.)));
        let envs = vec![e0, e1, e2, e3, e4, e5, e6, e7];

        let flatbush = Flatbush::new_unsorted(&envs, degree);

        // This is unsorted, so the order should be:
        // [e0..e3, e4..e7, p1=parent(e0..e3), p2=parent(e4..e7), root=parent(p1, p2)]

        assert_eq!(flatbush.degree, degree);
        assert_eq!(flatbush.level_indices, vec![0, 8, 12]);
        let expected_l0: Vec<(usize, Envelope<f32>)> =
            envs.clone().into_iter().enumerate().collect();
        assert_eq!(flatbush.tree[0..8], expected_l0[..]);
        assert_eq!(
            flatbush.tree[8..12],
            vec![
                (0, Envelope::from(((7.0, 44.), (99., 79.)))),
                (1, Envelope::from(((7.1, 40.), (108., 91.)))),
                (2, Envelope::Empty),
                (3, Envelope::Empty),
            ][..]
        );
        assert_eq!(
            flatbush.tree[12],
            (0, Envelope::from(((7., 40.,), (108., 91.))))
        );
    }

    fn get_envelopes() -> Vec<Envelope<f32>> {
        #[rustfmt::skip]
        let rects: Vec<f32> = vec![
             8, 62, 11, 66,
            57, 17, 57, 19,
            76, 26, 79, 29,
            36, 56, 38, 56,
            92, 77, 96, 80,
            87, 70, 90, 74,
            43, 41, 47, 43,
             0, 58,  2, 62,
            76, 86, 80, 89,
            27, 13, 27, 15,
            71, 63, 75, 67,
            25,  2, 27,  2,
            87,  6, 88,  6,
            22, 90, 23, 93,
            22, 89, 22, 93,
            57, 11, 61, 13,
            61, 55, 63, 56,
            17, 85, 21, 87,
            33, 43, 37, 43,
             6,  1,  7,  3,
            80, 87, 80, 87,
            23, 50, 26, 52,
            58, 89, 58, 89,
            12, 30, 15, 34,
            32, 58, 36, 61,
            41, 84, 44, 87,
            44, 18, 44, 19,
            13, 63, 15, 67,
            52, 70, 54, 74,
            57, 59, 58, 59,
            17, 90, 20, 92,
            48, 53, 52, 56,
             2, 68, 92, 72,
            26, 52, 30, 52,
            56, 23, 57, 26,
            88, 48, 88, 48,
            66, 13, 67, 15,
             7, 82,  8, 86,
            46, 68, 50, 68,
            37, 33, 38, 36,
             6, 15,  8, 18,
            85, 36, 89, 38,
            82, 45, 84, 48,
            12,  2, 16,  3,
            26, 15, 26, 16,
            55, 23, 59, 26,
            76, 37, 79, 39,
            86, 74, 90, 77,
            16, 75, 18, 78,
            44, 18, 45, 21,
            52, 67, 54, 71,
            59, 78, 62, 78,
            24,  5, 24,  8,
            64, 80, 64, 83,
            66, 55, 70, 55,
             0, 17,  2, 19,
            15, 71, 18, 74,
            87, 57, 87, 59,
             6, 34,  7, 37,
            34, 30, 37, 32,
            51, 19, 53, 19,
            72, 51, 73, 55,
            29, 45, 30, 45,
            94, 94, 96, 95,
             7, 22, 11, 24,
            86, 45, 87, 48,
            33, 62, 34, 65,
            18, 10, 21, 14,
            64, 66, 67, 67,
            64, 25, 65, 28,
            27,  4, 31,  6,
            84,  4, 85,  5,
            48, 80, 50, 81,
             1, 61,  3, 61,
            71, 89, 74, 92,
            40, 42, 43, 43,
            27, 64, 28, 66,
            46, 26, 50, 26,
            53, 83, 57, 87,
            14, 75, 15, 79,
            31, 45, 34, 45,
            89, 84, 92, 88,
            84, 51, 85, 53,
            67, 87, 67, 89,
            39, 26, 43, 27,
            47, 61, 47, 63,
            23, 49, 25, 53,
            12,  3, 14,  5,
            16, 50, 19, 53,
            63, 80, 64, 84,
            22, 63, 22, 64,
            26, 66, 29, 66,
             2, 15,  3, 15,
            74, 77, 77, 79,
            64, 11, 68, 11,
            38,  4, 39,  8,
            83, 73, 87, 77,
            85, 52, 89, 56,
            74, 60, 76, 63,
            62, 66, 65, 67,
        ]
        .into_iter()
        .map(|v| v as f32)
        .collect();
        rects
            .chunks(4)
            .map(|r| Envelope::from(((r[0], r[1]), (r[2], r[3]))))
            .collect()
    }

    #[test]
    fn test_intersection_candidates_unsorted() {
        let envelopes = get_envelopes();
        let f = Flatbush::new_unsorted(&envelopes, 16);
        let query_rect = Rect::from(((40., 40.), (60., 60.)));

        let brute_results = find_brute_intersections(query_rect, &envelopes);
        let mut rtree_results = f.find_intersection_candidates(query_rect);
        rtree_results.sort();
        assert_eq!(rtree_results, brute_results);
    }

    #[test]
    fn test_intersection_candidates_hilbert() {
        let envelopes = get_envelopes();
        let f = Flatbush::new(&envelopes, 16);
        let query_rect = Rect::from(((40., 40.), (60., 60.)));

        let brute_results = find_brute_intersections(query_rect, &envelopes);
        let mut rtree_results = f.find_intersection_candidates(query_rect);
        rtree_results.sort();
        assert_eq!(rtree_results, brute_results);
    }

    #[test]
    fn test_self_intersection_unsorted() {
        let envelopes: Vec<Envelope<f32>> = get_envelopes();
        let f = Flatbush::new_unsorted(&envelopes, 16);

        let brute_results = find_brute_self_intersections(&envelopes);
        let mut rtree_results = f.find_self_intersection_candidates();
        rtree_results.sort();
        assert_eq!(rtree_results, brute_results);
    }

    #[test]
    fn test_self_intersection_hilbert() {
        let envelopes: Vec<Envelope<f32>> = get_envelopes();
        let f = Flatbush::new(&envelopes, 16);

        let brute_results = find_brute_self_intersections(&envelopes);
        let mut rtree_results = f.find_self_intersection_candidates();
        rtree_results.sort();
        assert_eq!(rtree_results, brute_results);
    }

    #[test]
    fn test_rtree_intersection_unsorted() {
        let mut envelopes1 = get_envelopes();
        let n_envs = envelopes1.len();
        let envelopes2 = envelopes1.split_off(2 * envelopes1.len() / 3);
        assert_eq!(envelopes1.len() + envelopes2.len(), n_envs);

        let f1 = Flatbush::new_unsorted(&envelopes1, 16);
        let f2 = Flatbush::new_unsorted(&envelopes2, 16);
        let mut rtree_results = f1.find_other_rtree_intersection_candidates(&f2);
        rtree_results.sort();
        let brute_results = find_brute_cross_intersections(&envelopes1, &envelopes2);
        assert_eq!(rtree_results, brute_results);
    }

    #[test]
    fn test_rtree_intersection_hilbert() {
        let mut envelopes1 = get_envelopes();
        let n_envs = envelopes1.len();
        let envelopes2 = envelopes1.split_off(2 * envelopes1.len() / 3);
        assert_eq!(envelopes1.len() + envelopes2.len(), n_envs);

        let f1 = Flatbush::new(&envelopes1, 16);
        let f2 = Flatbush::new(&envelopes2, 16);
        let mut rtree_results = f1.find_other_rtree_intersection_candidates(&f2);
        rtree_results.sort();
        let brute_results = find_brute_cross_intersections(&envelopes1, &envelopes2);
        assert_eq!(rtree_results, brute_results);
    }

    #[test]
    fn test_rtree_intersection_with_empty() {
        let envelopes1 = get_envelopes();
        let f1 = Flatbush::new(&envelopes1, 16);
        let f2 = Flatbush::new_empty();
        let rtree_results = f1.find_other_rtree_intersection_candidates(&f2);
        assert_eq!(rtree_results, vec![]);
    }

    fn find_brute_intersections(
        query_rect: Rect<f32>,
        envelopes: &Vec<Envelope<f32>>,
    ) -> Vec<usize> {
        envelopes
            .iter()
            .enumerate()
            .filter(|(_, e)| e.intersects(query_rect.into()))
            .map(|(i, _)| i)
            .collect()
    }

    fn find_brute_self_intersections(envelopes: &Vec<Envelope<f32>>) -> Vec<(usize, usize)> {
        type EnumEnv = (usize, Envelope<f32>);
        let enum_envelopes: Vec<EnumEnv> = envelopes.clone().into_iter().enumerate().collect();
        let env_prod: Vec<(EnumEnv, EnumEnv)> =
            iproduct!(enum_envelopes.clone(), enum_envelopes).collect();
        env_prod
            .into_iter()
            .filter(|((i1, _), (i2, _))| i1 < i2)
            .filter(|((_, e1), (_, e2))| e1.intersects(*e2))
            .map(|((i1, _), (i2, _))| (i1, i2))
            .collect()
    }

    fn find_brute_cross_intersections(
        envelopes1: &Vec<Envelope<f32>>,
        envelopes2: &Vec<Envelope<f32>>,
    ) -> Vec<(usize, usize)> {
        type EnumEnv = (usize, Envelope<f32>);
        let envelopes1: Vec<EnumEnv> = envelopes1.clone().into_iter().enumerate().collect();
        let envelopes2: Vec<EnumEnv> = envelopes2.clone().into_iter().enumerate().collect();
        iproduct!(envelopes1, envelopes2)
            .filter(|((_, e1), (_, e2))| e1.intersects(*e2))
            .map(|((i1, _), (i2, _))| (i1, i2))
            .collect()
    }
}
