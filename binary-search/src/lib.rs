/*
Copied from https://exercism.org/tracks/rust/exercises/binary-search/solutions/denenr

How do we have a recursive find function that doesn't keep track of the index?
Typically recursive definitions will carry over the index in one of its arguments.

The problem:
When the algorithm determines that the value we're looking for is in the last half of the slice,
we run the algorithm again on a new slice containing only the last half of the original slice.
The new slice knows nothing about the indexes of the values in the original slice, so if the
value is found in it, the returned index will refer to its position in the new slice.

The trick:
Since we know how far into the original slice the new slice started, we can
just add that offset to the returned index (if there is one) to get the original index!
That offset is encoded in the signature of the new slice: &collection[mid + 1..].

The value at index mid + 1 in the original slice will have index 0 in the new slice,
so to get the original index, we have to add mid + 1. We have this in the form of |i| i + mid + 1.
*/

use std::cmp::Ordering;

pub fn find<C, T>(collection: C, value: T) -> Option<usize>
where
    C: AsRef<[T]>,
    T: Ord,
{
    let collection = collection.as_ref();
    let mid = collection.len() / 2;

    match value.cmp(collection.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&collection[..mid], value),
        Ordering::Greater => find(&collection[mid + 1..], value).map(|i| i + mid + 1),
    }
}
