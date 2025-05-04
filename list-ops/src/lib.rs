use std::iter;

pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    iter::from_fn(move || a.next().or_else(|| b.next()))
}

pub fn concat<I>(mut nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut current = nested_iter.next();
    iter::from_fn(move || {
        while let Some(ref mut iter) = current {
            if let Some(item) = iter.next() {
                return Some(item);
            }
            current = nested_iter.next()
        }
        None
    })
}

pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    iter::from_fn(move || iter.find(|item| predicate(item)))
}

pub fn length<I: Iterator>(iter: I) -> usize {
    foldl(iter, 0, |count, _| count + 1)
}

pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    iter::from_fn(move || iter.next().map(&function))
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    for item in iter {
        acc = function(acc, item);
    }
    acc
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    while let Some(item) = iter.next_back() {
        acc = function(acc, item);
    }
    acc
}

pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    iter::from_fn(move || iter.next_back())
}
