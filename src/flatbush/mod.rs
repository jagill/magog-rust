/**
 * A fast, low memory footprint static Rtree.
 *
 * Original implementation in Javascript: https://github.com/mourner/flatbush
 * Initial conversion to rust by Jacob Wasserman @jwass
 */
use num_traits::PrimInt;
use rayon::prelude::*;

mod hilbert;

use crate::primitives::{Coordinate, Envelope, HasEnvelope};
use hilbert::Hilbert;

const DEFAULT_DEGREE: usize = 8;

pub struct Flatbush<'a, C, E>
where
    C: Coordinate,
    E: HasEnvelope<C>,
{
    degree: usize,
    // nodes in level i are (level_indices[i] .. level_indices[i + 1] - 1)
    level_indices: Vec<usize>,
    tree: Vec<(usize, Envelope<C>)>,
    items: &'a Vec<E>,
}

impl<'a, C, E> Flatbush<'a, C, E>
where
    C: Coordinate,
    E: HasEnvelope<C>,
{
    pub fn new_empty(items: &'a Vec<E>) -> Flatbush<'a, C, E> {
        Flatbush {
            degree: DEFAULT_DEGREE,
            level_indices: Vec::new(),
            tree: Vec::new(),
            items,
        }
    }

    pub fn new(items: &Vec<E>, degree: usize) -> Flatbush<C, E> {
        let total_envelope = Envelope::from_envelopes(items.iter().map(|e| e.envelope()));
        if total_envelope.rect == None {
            // The list of items are empty, or all items are empty.
            return Flatbush::new_unsorted(items, degree);
        }
        let hilbert_square = Hilbert::new(total_envelope.rect.unwrap());

        let mut entries: Vec<(u32, usize, Envelope<C>)> = items
            .iter()
            .map(|e| e.envelope())
            .enumerate()
            .map(|(i, e)| (hilbert_square.safe_hilbert(e), i, e))
            .collect();

        entries.par_sort_unstable_by_key(|&(h, _, _)| h);

        Flatbush::_new_unsorted(
            items,
            entries.into_iter().map(|(_, i, e)| (i, e)).collect(),
            degree,
        )
    }

    pub fn new_unsorted(items: &Vec<E>, degree: usize) -> Flatbush<C, E> {
        let entries = items.iter().map(|e| e.envelope()).enumerate().collect();
        Flatbush::_new_unsorted(items, entries, degree)
    }

    fn _new_unsorted(
        items: &Vec<E>,
        entries: Vec<(usize, Envelope<C>)>,
        degree: usize,
    ) -> Flatbush<C, E> {
        // This should always be true, since we are calling it internally.
        assert_eq!(items.len(), entries.len());

        if degree != degree.next_power_of_two() {
            panic!("Degree must be a positive power of 2.");
        }
        let degree_exp = degree.trailing_zeros();

        if items.len() == 0 {
            return Flatbush::new_empty(items);
        }

        let mut tree: Vec<(usize, Envelope<C>)> = Vec::with_capacity(3 * items.len() / 2);
        tree.extend(entries.iter());

        let estimated_capacity = quick_log_ceil(items.len(), degree_exp) + 1;
        let mut level_indices: Vec<usize> = Vec::with_capacity(estimated_capacity as usize);
        level_indices.push(0);

        let mut level = 0;
        let mut level_size = items.len();
        let mut level_capacity;

        while level_size > 1 {
            level_capacity = next_multiple(level_size, degree);
            level_indices.push(level_indices[level] + level_capacity);
            // Pad out the remaining spaces with empties that will never match.
            let mut dummy_index = level_size;
            while tree.len() < level_indices[level + 1] {
                tree.push((dummy_index, Envelope::empty()));
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
            items,
        }
    }

    /**
     * Get a start_index for children of a parent at `level` with `index`.
     *
     * The children will be at indices start_index..(start_index + DEGREE)
     * All of the children will be defined, although some may be empty.
     */
    fn get_first_child_index(&self, level: usize, index: usize) -> usize {
        self.level_indices[level - 1] + index * self.degree
    }
}

/**
 * Take a quick ceil(log(n, 2**e)).
 *
 * n is the number to take the log of.
 * e is the exponent of the base.
 * We define quick_log_ceil(0, e) == 0.
 * quick_log_ceil(n, 0) is illegal and will cause a panic.
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
    use super::{div_ceil, next_multiple, quick_log_ceil, Envelope, Flatbush};

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
    fn test_build_tree_unordered() {
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
                (2, Envelope::empty()),
                (3, Envelope::empty()),
            ][..]
        );
        assert_eq!(
            flatbush.tree[12],
            (0, Envelope::from(((7., 40.,), (108., 91.))))
        );
    }

    fn get_envelopes() -> Vec<Envelope<f32>> {
        let rects: Vec<f32> = vec![
            8, 62, 11, 66, 57, 17, 57, 19, 76, 26, 79, 29, 36, 56, 38, 56, 92, 77, 96, 80, 87, 70,
            90, 74, 43, 41, 47, 43, 0, 58, 2, 62, 76, 86, 80, 89, 27, 13, 27, 15, 71, 63, 75, 67,
            25, 2, 27, 2, 87, 6, 88, 6, 22, 90, 23, 93, 22, 89, 22, 93, 57, 11, 61, 13, 61, 55, 63,
            56, 17, 85, 21, 87, 33, 43, 37, 43, 6, 1, 7, 3, 80, 87, 80, 87, 23, 50, 26, 52, 58, 89,
            58, 89, 12, 30, 15, 34, 32, 58, 36, 61, 41, 84, 44, 87, 44, 18, 44, 19, 13, 63, 15, 67,
            52, 70, 54, 74, 57, 59, 58, 59, 17, 90, 20, 92, 48, 53, 52, 56, 92, 68, 92, 72, 26, 52,
            30, 52, 56, 23, 57, 26, 88, 48, 88, 48, 66, 13, 67, 15, 7, 82, 8, 86, 46, 68, 50, 68,
            37, 33, 38, 36, 6, 15, 8, 18, 85, 36, 89, 38, 82, 45, 84, 48, 12, 2, 16, 3, 26, 15, 26,
            16, 55, 23, 59, 26, 76, 37, 79, 39, 86, 74, 90, 77, 16, 75, 18, 78, 44, 18, 45, 21, 52,
            67, 54, 71, 59, 78, 62, 78, 24, 5, 24, 8, 64, 80, 64, 83, 66, 55, 70, 55, 0, 17, 2, 19,
            15, 71, 18, 74, 87, 57, 87, 59, 6, 34, 7, 37, 34, 30, 37, 32, 51, 19, 53, 19, 72, 51,
            73, 55, 29, 45, 30, 45, 94, 94, 96, 95, 7, 22, 11, 24, 86, 45, 87, 48, 33, 62, 34, 65,
            18, 10, 21, 14, 64, 66, 67, 67, 64, 25, 65, 28, 27, 4, 31, 6, 84, 4, 85, 5, 48, 80, 50,
            81, 1, 61, 3, 61, 71, 89, 74, 92, 40, 42, 43, 43, 27, 64, 28, 66, 46, 26, 50, 26, 53,
            83, 57, 87, 14, 75, 15, 79, 31, 45, 34, 45, 89, 84, 92, 88, 84, 51, 85, 53, 67, 87, 67,
            89, 39, 26, 43, 27, 47, 61, 47, 63, 23, 49, 25, 53, 12, 3, 14, 5, 16, 50, 19, 53, 63,
            80, 64, 84, 22, 63, 22, 64, 26, 66, 29, 66, 2, 15, 3, 15, 74, 77, 77, 79, 64, 11, 68,
            11, 38, 4, 39, 8, 83, 73, 87, 77, 85, 52, 89, 56, 74, 60, 76, 63, 62, 66, 65, 67,
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
    fn test_intersection_candidates() {
        let envelopes = get_envelopes();
        let f = Flatbush::new(&envelopes, 16);

        /*
        let results = f.search(&Rect {
            min_x: 40.,
            min_y: 40.,
            max_x: 60.,
            max_y: 60.,
        });
        let mut rrects: Vec<f32> = Vec::new();
        for i in results {
            rrects.push(rects[4 * i]);
            rrects.push(rects[4 * i + 1]);
            rrects.push(rects[4 * i + 2]);
            rrects.push(rects[4 * i + 3]);
        }
        println!("r: {:?}", rrects);
        */
    }
}
