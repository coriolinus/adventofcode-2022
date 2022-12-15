use std::{
    borrow::Borrow,
    fmt,
    ops::{Add, RangeInclusive, Sub},
};

use num_traits::One;

/// How many points are contained within the bounds of this range.
pub fn contained_points<T>(range: RangeInclusive<T>) -> u64
where
    T: PartialOrd + Sub<Output = T>,
    u64: TryFrom<T>,
    <u64 as TryFrom<T>>::Error: std::fmt::Debug,
{
    let (low, high) = range.into_inner();
    if low <= high {
        let diff: u64 = (high - low).try_into().expect("out of bounds of u64");
        diff + 1
    } else {
        0
    }
}

/// Low value of a range.
///
/// Given that `T` is `Copy`, this should be inexpensive.
#[inline]
fn low<R, T>(range: R) -> T
where
    R: Borrow<RangeInclusive<T>>,
    T: Copy,
{
    range.borrow().clone().into_inner().0
}

/// High value of a range.
///
/// Given that `T` is `Copy`, this should be inexpensive.
#[inline]
fn high<R, T>(range: R) -> T
where
    R: Borrow<RangeInclusive<T>>,
    T: Copy,
{
    range.borrow().clone().into_inner().1
}

/// `true` when `a` is lower than `b`.
#[inline]
fn is_lower<R, T>(a: R, b: R) -> bool
where
    R: Borrow<RangeInclusive<T>>,
    T: Copy + PartialOrd,
{
    low(a) <= low(b)
}

/// `true` when this slice of ranges is sorted by low bound
#[cfg(debug_assertions)]
fn is_sorted<T>(ranges: &[RangeInclusive<T>]) -> bool
where
    T: Copy + PartialOrd,
{
    ranges
        .windows(2)
        .all(|window| is_lower(&window[0], &window[1]))
}

/// `true` if `a` and `b` overlap.
///
/// `a` must be lower than `b`.
fn overlaps<R, T>(a: R, b: R) -> bool
where
    R: Borrow<RangeInclusive<T>>,
    T: Copy + PartialOrd,
{
    let a = a.borrow();
    let b = b.borrow();
    debug_assert!(is_lower(a, b));
    high(a) >= low(b)
}

/// `true` if there is no pair of adjacent ranges which overlap
#[cfg(debug_assertions)]
fn no_overlaps<T>(ranges: &[RangeInclusive<T>]) -> bool
where
    T: Copy + PartialOrd,
{
    !ranges
        .windows(2)
        .any(|window| overlaps(&window[0], &window[1]))
}

/// Insert a new range in sorted position by low index.
///
/// This may lead to overlaps.
///
/// Return the index at which the range was inserted.
fn insert<T>(ranges: &mut Vec<RangeInclusive<T>>, new_range: RangeInclusive<T>) -> usize
where
    T: Copy + Ord,
{
    let index = ranges
        .binary_search_by_key(&low(&new_range), |range| low(range))
        .unwrap_or_else(|idx| idx);
    ranges.insert(index, new_range);
    debug_assert!(is_sorted(ranges), "ranges are sorted on insertion");
    index
}

/// Coalesce as many ranges as required into `ranges[idx]`, shifting the rest left.
fn deoverlap<T>(ranges: &mut Vec<RangeInclusive<T>>, idx: usize)
where
    T: Copy + Ord,
{
    // it is possible that inserting a range at position 1 has caused a new overlap with position 0,
    // so we need to check from one step lower.
    let mut idx = idx.checked_sub(1).unwrap_or_default();
    let Some(mut high_bound) = ranges.get(idx).map(high) else {
        // ranges is empty
        return;
    };

    let mut upper_idx = idx;
    let mut first = true;
    while let Some(next_range) = ranges.get(upper_idx + 1) {
        if high_bound >= low(next_range) {
            // we've discovered an overlap!
            // update, and let's check again if we've overlapped again.
            upper_idx += 1;
            high_bound = high_bound.max(high(next_range));
        } else if first {
            // reset to check again at the nominal index
            idx += 1;
            upper_idx = idx;
            high_bound = high(next_range);
        } else {
            // no more overlaps!
            break;
        }
        first = false;
    }

    if upper_idx == idx {
        // no overlaps
        return;
    }

    let new_range = low(&ranges[idx])..=high_bound;
    ranges.drain(idx..=upper_idx);
    ranges.insert(idx, new_range);

    debug_assert!(
        no_overlaps(ranges),
        "ranges have no overlaps after deoverlapping"
    );
}

/// Merge the provided ranges into a minimal sorted set of distinct ranges.
pub fn merge_ranges<T>(
    ranges: impl IntoIterator<Item = RangeInclusive<T>>,
) -> Vec<RangeInclusive<T>>
where
    T: Copy + Ord,
{
    let ranges = ranges.into_iter();
    let (min, _max) = ranges.size_hint();

    let ranges = ranges.fold(
        Vec::with_capacity(min),
        |mut ranges: Vec<RangeInclusive<T>>, new_range| {
            let idx = insert(&mut ranges, new_range);
            deoverlap(&mut ranges, idx);
            ranges
        },
    );

    debug_assert!(is_sorted(&ranges), "ranges are sorted");
    debug_assert!(no_overlaps(&ranges), "range boundaries overlap");

    ranges
}

/// Find the single point which is inside `bounds` but outside of any of `impossible`.
///
/// `impossible` should be sorted and non-overlapping.
pub fn find_excluded<T>(bounds: &RangeInclusive<T>, impossible: &[RangeInclusive<T>]) -> Option<T>
where
    T: Copy + Ord + One + Add<Output = T> + fmt::Display,
{
    debug_assert!(is_sorted(impossible));
    debug_assert!(no_overlaps(impossible));

    let mut found = Vec::with_capacity(4);
    let mut cursor = low(bounds);

    for range in impossible {
        if !bounds.contains(&cursor) {
            // we've exhausted the set of these bounds
            break;
        }
        if range.contains(&cursor) {
            cursor = high(range) + T::one();
        } else if low(range) == cursor + T::one() {
            found.push(cursor);
            cursor = high(range);
        } else {
            eprintln!("found an empty range: {cursor}..{}", low(range));
            return None;
        }
    }

    if found.len() > 1 {
        eprintln!("found several points outside the ranges");
    }
    (found.len() == 1).then(|| found[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::empty([], &[])]
    #[case::one_item([0..=0], &[0..=0])]
    #[case::two_items_sorted_no_overlap([0..=0, 1..=1], &[0..=0, 1..=1])]
    #[case::two_items_reversed_no_overlap([1..=1, 0..=0], &[0..=0, 1..=1])]
    #[case::two_items_entirely_contained([0..=2, 1..=1], &[0..=2])]
    #[case::two_items_low_overlap([1..=3, 0..=2], &[0..=3])]
    #[case::two_items_high_overlap([0..=2, 1..=3], &[0..=3])]
    #[case::three_items_big_overlap([1..=1, 2..=2, 0..=3], &[0..=3])]
    fn test_merge_ranges(
        #[case] ranges: impl IntoIterator<Item = RangeInclusive<u32>>,
        #[case] expect: &[RangeInclusive<u32>],
    ) {
        let merged = merge_ranges(ranges);
        dbg!(&merged, &expect);
        assert_eq!(merged, expect);
    }

    #[test]
    fn test_example() {
        let mut ranges = vec![-667788..=1480842, 1989987..=4025721];
        let to_insert = 1581451..=2191623;

        assert!(is_sorted(&ranges));
        assert!(no_overlaps(&ranges));

        let idx = insert(&mut ranges, to_insert);
        assert_eq!(idx, 1);
        deoverlap(&mut ranges, idx);

        assert!(is_sorted(&ranges));
        assert!(no_overlaps(&ranges));
    }
}
