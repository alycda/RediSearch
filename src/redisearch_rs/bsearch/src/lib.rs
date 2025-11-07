/*
 * Copyright (c) 2006-Present, Redis Ltd.
 * All rights reserved.
 *
 * Licensed under your choice of the Redis Source Available License 2.0
 * (RSALv2); or (b) the Server Side Public License v1 (SSPLv1); or (c) the
 * GNU Affero General Public License v3 (AGPLv3).
*/

//! # Binary Search Utilities for Range Queries
//!
//! This crate provides specialized binary search functions optimized for range queries
//! on sorted arrays. Unlike standard binary search which finds exact matches, these
//! functions locate boundary positions for range queries.
//!
//! ## Use Case
//!
//! When implementing range queries (e.g., finding all elements between A and B), you need:
//! - The index of the first element >= A (lower bound)
//! - The index of the first element > B (upper bound)
//!
//! ## Functions
//!
//! - [`bsearch_ge`] - Find first element greater than or equal to target (>=)
//! - [`bsearch_le`] - Find last element less than or equal to target (<=)
//! - [`bsearch_eq`] - Find exact match, or return None
//!
//! ## Example: Range Query
//!
//! ```
//! use bsearch::{bsearch_ge, bsearch_le};
//!
//! let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
//!
//! // Find all elements in range [25, 75]
//! let start_idx = bsearch_ge(&data, &25, |a, b| a.cmp(b)); // First element >= 25
//! let end_idx = bsearch_le(&data, &75, |a, b| a.cmp(b));   // Last element <= 75
//!
//! if let (Some(start), Some(end)) = (start_idx, end_idx) {
//!     let range = &data[start..=end];
//!     assert_eq!(range, &[30, 40, 50, 60, 70]);
//! }
//! ```
//!
//! ## Performance
//!
//! All functions run in O(log n) time with minimal overhead. The implementation
//! uses Rust's standard library partition_point and binary_search_by for optimal performance and correctness.

use std::cmp::Ordering;

/// Find the index of the first element greater than or equal to the target.
///
/// This is also known as finding the "lower bound" in range query terminology.
///
/// # Arguments
///
/// * `arr` - The sorted array to search
/// * `target` - The value to search for
/// * `cmp` - Comparison function that returns the ordering of two elements
///
/// # Returns
///
/// - `Some(index)` - Index of first element >= target
/// - `None` - If all elements are < target (target would go at end)
///
/// # Examples
///
/// ```
/// use bsearch::bsearch_ge;
///
/// let data = vec![10, 20, 30, 40, 50];
///
/// // Exact match
/// assert_eq!(bsearch_ge(&data, &30, |a, b| a.cmp(b)), Some(2));
///
/// // Between elements - returns next higher
/// assert_eq!(bsearch_ge(&data, &35, |a, b| a.cmp(b)), Some(3));
///
/// // Before first element
/// assert_eq!(bsearch_ge(&data, &5, |a, b| a.cmp(b)), Some(0));
///
/// // After last element
/// assert_eq!(bsearch_ge(&data, &100, |a, b| a.cmp(b)), None);
/// ```
pub fn bsearch_ge<T, F>(arr: &[T], target: &T, cmp: F) -> Option<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    let idx = arr.partition_point(|elem| cmp(elem, target) == Ordering::Less);
    (idx < arr.len()).then_some(idx)
}

/// Find the index of the last element less than or equal to the target.
///
/// This is useful for finding the "upper bound" in range queries.
///
/// # Arguments
///
/// * `arr` - The sorted array to search
/// * `target` - The value to search for
/// * `cmp` - Comparison function that returns the ordering of two elements
///
/// # Returns
///
/// - `Some(index)` - Index of last element <= target
/// - `None` - If all elements are > target (target would go before start)
///
/// # Examples
///
/// ```
/// use bsearch::bsearch_le;
///
/// let data = vec![10, 20, 30, 40, 50];
///
/// // Exact match
/// assert_eq!(bsearch_le(&data, &30, |a, b| a.cmp(b)), Some(2));
///
/// // Between elements - returns previous lower
/// assert_eq!(bsearch_le(&data, &35, |a, b| a.cmp(b)), Some(2));
///
/// // After last element
/// assert_eq!(bsearch_le(&data, &100, |a, b| a.cmp(b)), Some(4));
///
/// // Before first element
/// assert_eq!(bsearch_le(&data, &5, |a, b| a.cmp(b)), None);
/// ```
pub fn bsearch_le<T, F>(arr: &[T], target: &T, cmp: F) -> Option<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    let idx = arr.partition_point(|elem| cmp(elem, target) != Ordering::Greater);
    idx.checked_sub(1)
}

/// Find the exact index of an element equal to the target.
///
/// This is a standard binary search for exact matches.
///
/// # Arguments
///
/// * `arr` - The sorted array to search
/// * `target` - The value to search for
/// * `cmp` - Comparison function that returns the ordering of two elements
///
/// # Returns
///
/// - `Some(index)` - Index of an element equal to target
/// - `None` - If no exact match exists
///
/// # Examples
///
/// ```
/// use bsearch::bsearch_eq;
///
/// let data = vec![10, 20, 30, 40, 50];
///
/// // Exact match
/// assert_eq!(bsearch_eq(&data, &30, |a, b| a.cmp(b)), Some(2));
///
/// // No match
/// assert_eq!(bsearch_eq(&data, &35, |a, b| a.cmp(b)), None);
/// ```

pub fn bsearch_eq<T, F>(arr: &[T], target: &T, cmp: F) -> Option<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    arr.binary_search_by(|elem| cmp(elem, target)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsearch_ge_exact_matches() {
        let data = vec![10, 20, 30, 40, 50];

        assert_eq!(bsearch_ge(&data, &10, |a, b| a.cmp(b)), Some(0));
        assert_eq!(bsearch_ge(&data, &20, |a, b| a.cmp(b)), Some(1));
        assert_eq!(bsearch_ge(&data, &30, |a, b| a.cmp(b)), Some(2));
        assert_eq!(bsearch_ge(&data, &40, |a, b| a.cmp(b)), Some(3));
        assert_eq!(bsearch_ge(&data, &50, |a, b| a.cmp(b)), Some(4));
    }

    #[test]
    fn test_bsearch_ge_between_elements() {
        let data = vec![10, 20, 30, 40, 50];

        assert_eq!(bsearch_ge(&data, &15, |a, b| a.cmp(b)), Some(1)); // -> 20
        assert_eq!(bsearch_ge(&data, &25, |a, b| a.cmp(b)), Some(2)); // -> 30
        assert_eq!(bsearch_ge(&data, &35, |a, b| a.cmp(b)), Some(3)); // -> 40
        assert_eq!(bsearch_ge(&data, &45, |a, b| a.cmp(b)), Some(4)); // -> 50
    }

    #[test]
    fn test_bsearch_ge_boundaries() {
        let data = vec![10, 20, 30, 40, 50];

        assert_eq!(bsearch_ge(&data, &5, |a, b| a.cmp(b)), Some(0));   // Before first
        assert_eq!(bsearch_ge(&data, &100, |a, b| a.cmp(b)), None);    // After last
    }

    #[test]
    fn test_bsearch_ge_empty() {
        let data: Vec<i32> = vec![];
        assert_eq!(bsearch_ge(&data, &10, |a, b| a.cmp(b)), None);
    }

    #[test]
    fn test_bsearch_ge_single_element() {
        let data = vec![42];
        assert_eq!(bsearch_ge(&data, &20, |a, b| a.cmp(b)), Some(0));
        assert_eq!(bsearch_ge(&data, &42, |a, b| a.cmp(b)), Some(0));
        assert_eq!(bsearch_ge(&data, &50, |a, b| a.cmp(b)), None);
    }

    #[test]
    fn test_bsearch_le_exact_matches() {
        let data = vec![10, 20, 30, 40, 50];

        assert_eq!(bsearch_le(&data, &10, |a, b| a.cmp(b)), Some(0));
        assert_eq!(bsearch_le(&data, &20, |a, b| a.cmp(b)), Some(1));
        assert_eq!(bsearch_le(&data, &30, |a, b| a.cmp(b)), Some(2));
        assert_eq!(bsearch_le(&data, &40, |a, b| a.cmp(b)), Some(3));
        assert_eq!(bsearch_le(&data, &50, |a, b| a.cmp(b)), Some(4));
    }

    #[test]
    fn test_bsearch_le_between_elements() {
        let data = vec![10, 20, 30, 40, 50];

        assert_eq!(bsearch_le(&data, &15, |a, b| a.cmp(b)), Some(0)); // -> 10
        assert_eq!(bsearch_le(&data, &25, |a, b| a.cmp(b)), Some(1)); // -> 20
        assert_eq!(bsearch_le(&data, &35, |a, b| a.cmp(b)), Some(2)); // -> 30
        assert_eq!(bsearch_le(&data, &45, |a, b| a.cmp(b)), Some(3)); // -> 40
    }

    #[test]
    fn test_bsearch_le_boundaries() {
        let data = vec![10, 20, 30, 40, 50];

        assert_eq!(bsearch_le(&data, &5, |a, b| a.cmp(b)), None);      // Before first
        assert_eq!(bsearch_le(&data, &100, |a, b| a.cmp(b)), Some(4)); // After last
    }

    #[test]
    fn test_bsearch_le_empty() {
        let data: Vec<i32> = vec![];
        assert_eq!(bsearch_le(&data, &10, |a, b| a.cmp(b)), None);
    }

    #[test]
    fn test_bsearch_le_single_element() {
        let data = vec![42];
        assert_eq!(bsearch_le(&data, &20, |a, b| a.cmp(b)), None);
        assert_eq!(bsearch_le(&data, &42, |a, b| a.cmp(b)), Some(0));
        assert_eq!(bsearch_le(&data, &50, |a, b| a.cmp(b)), Some(0));
    }

    #[test]
    fn test_bsearch_eq_exact_matches() {
        let data = vec![10, 20, 30, 40, 50];

        assert_eq!(bsearch_eq(&data, &10, |a, b| a.cmp(b)), Some(0));
        assert_eq!(bsearch_eq(&data, &20, |a, b| a.cmp(b)), Some(1));
        assert_eq!(bsearch_eq(&data, &30, |a, b| a.cmp(b)), Some(2));
        assert_eq!(bsearch_eq(&data, &40, |a, b| a.cmp(b)), Some(3));
        assert_eq!(bsearch_eq(&data, &50, |a, b| a.cmp(b)), Some(4));
    }

    #[test]
    fn test_bsearch_eq_no_match() {
        let data = vec![10, 20, 30, 40, 50];

        assert_eq!(bsearch_eq(&data, &15, |a, b| a.cmp(b)), None);
        assert_eq!(bsearch_eq(&data, &25, |a, b| a.cmp(b)), None);
        assert_eq!(bsearch_eq(&data, &5, |a, b| a.cmp(b)), None);
        assert_eq!(bsearch_eq(&data, &100, |a, b| a.cmp(b)), None);
    }

    #[test]
    fn test_bsearch_eq_empty() {
        let data: Vec<i32> = vec![];
        assert_eq!(bsearch_eq(&data, &10, |a, b| a.cmp(b)), None);
    }

    #[test]
    fn test_range_query_example() {
        let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];

        // Find all elements in range [25, 75]
        let start_idx = bsearch_ge(&data, &25, |a, b| a.cmp(b));
        let end_idx = bsearch_le(&data, &75, |a, b| a.cmp(b));

        assert_eq!(start_idx, Some(2)); // Index of 30
        assert_eq!(end_idx, Some(6));   // Index of 70

        if let (Some(start), Some(end)) = (start_idx, end_idx) {
            let range = &data[start..=end];
            assert_eq!(range, &[30, 40, 50, 60, 70]);
        }
    }

    #[test]
    fn test_large_array() {
        let data: Vec<i32> = (0..10000).map(|i| i * 2).collect();

        // Test exact match
        assert_eq!(bsearch_eq(&data, &1000, |a, b| a.cmp(b)), Some(500));

        // Test range query [100, 200]
        let start = bsearch_ge(&data, &100, |a, b| a.cmp(b));
        let end = bsearch_le(&data, &200, |a, b| a.cmp(b));
        assert_eq!(start, Some(50));
        assert_eq!(end, Some(100));
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_bsearch_ge_is_correct(
            mut data in prop::collection::vec(any::<i32>(), 0..100),
            target in any::<i32>()
        ) {
            data.sort_unstable();
            data.dedup();

            let result = bsearch_ge(&data, &target, |a, b| a.cmp(b));

            match result {
                Some(idx) => {
                    // Element at idx should be >= target
                    prop_assert!(data[idx] >= target);
                    // All elements before idx should be < target
                    for i in 0..idx {
                        prop_assert!(data[i] < target);
                    }
                }
                None => {
                    // All elements should be < target
                    for elem in &data {
                        prop_assert!(elem < &target);
                    }
                }
            }
        }

        #[test]
        fn prop_bsearch_le_is_correct(
            mut data in prop::collection::vec(any::<i32>(), 0..100),
            target in any::<i32>()
        ) {
            data.sort_unstable();
            data.dedup();

            let result = bsearch_le(&data, &target, |a, b| a.cmp(b));

            match result {
                Some(idx) => {
                    // Element at idx should be <= target
                    prop_assert!(data[idx] <= target);
                    // All elements after idx should be > target
                    for i in (idx + 1)..data.len() {
                        prop_assert!(data[i] > target);
                    }
                }
                None => {
                    // All elements should be > target
                    for elem in &data {
                        prop_assert!(elem > &target);
                    }
                }
            }
        }

        #[test]
        fn prop_bsearch_eq_is_correct(
            mut data in prop::collection::vec(any::<i32>(), 0..100),
            target in any::<i32>()
        ) {
            data.sort_unstable();
            data.dedup();

            let result = bsearch_eq(&data, &target, |a, b| a.cmp(b));

            match result {
                Some(idx) => {
                    prop_assert_eq!(data[idx], target);
                }
                None => {
                    prop_assert!(!data.contains(&target));
                }
            }
        }

        #[test]
        fn prop_range_query_covers_range(
            mut data in prop::collection::vec(any::<i32>(), 0..100),
            range_start in any::<i32>(),
            range_len in 0..1000i32
        ) {
            data.sort_unstable();
            data.dedup();

            let range_end = range_start.saturating_add(range_len);

            let start_idx = bsearch_ge(&data, &range_start, |a, b| a.cmp(b));
            let end_idx = bsearch_le(&data, &range_end, |a, b| a.cmp(b));

            if let (Some(start), Some(end)) = (start_idx, end_idx) {
                if start <= end {
                    let range_elements = &data[start..=end];
                    // All elements in range should be within bounds
                    for elem in range_elements {
                        prop_assert!(*elem >= range_start);
                        prop_assert!(*elem <= range_end);
                    }
                }
            }
        }
    }
}
