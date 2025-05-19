use std::collections::{HashMap, HashSet};

static BOOK_COST: f64 = 8.0;

pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }

    let nsets = count_sets(books);

    *[
        price_for_largest_sets(books, nsets),
        price_for_balanced_sets(books, nsets),
        price_for_sets_of_four(books, nsets).unwrap_or(u32::MAX),
    ]
    .iter()
    .min()
    .unwrap()
}

fn price_for_largest_sets(books: &[u32], nsets: usize) -> u32 {
    let mut sets = vec![HashSet::new(); nsets];

    for book in books {
        let set = sets
            .iter_mut()
            .find(|set| !set.contains(book))
            .expect("should have one available set");
        set.insert(book);
    }

    calculate_price(sets)
}

fn price_for_balanced_sets(books: &[u32], nsets: usize) -> u32 {
    let mut sets = vec![Vec::new(); nsets];

    for (idx, book) in books.iter().enumerate() {
        sets[idx % nsets].push(book);
    }

    calculate_price(sets)
}

fn price_for_sets_of_four(books: &[u32], nsets: usize) -> Option<u32> {
    let mut sets = vec![HashSet::new(); nsets];

    let tally = tally(books);
    let sorted = sorted_books(books, &tally);

    for book in sorted {
        let set = sets
            .iter_mut()
            .find(|set| !set.contains(&book) && set.len() < 4)?;
        set.insert(book);
    }

    Some(calculate_price(sets))
}

fn count_sets(books: &[u32]) -> usize {
    let tally = tally(books);
    let sorted = sorted_books(books, &tally);
    let most_frequent = sorted.first().unwrap();

    tally[most_frequent]
}

fn tally(books: &[u32]) -> HashMap<&u32, usize> {
    books.iter().fold(HashMap::new(), |mut tally, book| {
        *tally.entry(book).or_insert(0) += 1;
        tally
    })
}

fn sorted_books(books: &[u32], tally: &HashMap<&u32, usize>) -> Vec<u32> {
    let mut sorted = books.to_vec();
    sorted.sort_unstable_by_key(|book| std::cmp::Reverse(tally[&book]));

    sorted
}

fn calculate_price<T>(sets: Vec<T>) -> u32
where
    T: IntoIterator + Clone,
{
    sets.iter()
        .map(|set| {
            let nbooks = set.clone().into_iter().count();
            let discount: f64 = match nbooks {
                1 => 0.0,
                2 => 5.0,
                3 => 10.0,
                4 => 20.0,
                5 => 25.0,
                _ => unreachable!(),
            };

            ((nbooks as f64 * BOOK_COST * (100.0 - discount) / 100.0) * 100.0) as u32
        })
        .sum()
}
