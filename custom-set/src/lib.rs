use std::hash::{DefaultHasher, Hash, Hasher};
/*
Inspired by the hash table implementation in
Crafting Interpreters:
https://craftinginterpreters.com/hash-tables.html

Uses linear probing for collision resolution and hash-based
indexing for fast lookup. Dynamic resizing is used to minimize allocations.
This is a simple approach, but maintains the expected
runtimes for standard set implementations:

- The basic operations (add, contains, is_empty) maintain expected O(1) average-case complexity
- The set operations (union, intersection, difference) maintain expected linear complexity
- The comparison operations (is_subset, is_disjoint) maintain expected linear complexity
*/

const TABLE_MAX_LOAD: f32 = 0.75;

#[derive(Debug)]
pub struct CustomSet<T: Hash + Eq + Clone> {
    entries: Vec<Option<T>>,
    count: usize,
    capacity: usize,
}

impl<T: Hash + Eq + Clone> CustomSet<T> {
    pub fn new(items: &[T]) -> Self {
        let mut set = CustomSet {
            entries: Vec::new(),
            count: 0,
            capacity: 0,
        };

        for item in items {
            set.add(item.clone());
        }

        set
    }

    pub fn add(&mut self, key: T) -> bool {
        // Dynamic resizing
        if self.count + 1 > (self.capacity as f32 * TABLE_MAX_LOAD) as usize {
            let capacity = if self.capacity == 0 {
                8 // Not too small, not too big
            } else {
                self.capacity.next_power_of_two()
            };
            self.adjust_capacity(capacity);
        }

        let index = self.find_entry_index(&key);
        let is_new_key = match &self.entries[index] {
            None => true,
            Some(existing) => existing != &key,
        };

        if is_new_key {
            self.count += 1;
            self.entries[index] = Some(key);
        }

        is_new_key
    }

    pub fn contains(&self, key: &T) -> bool {
        if self.count == 0 {
            return false;
        }

        let index = self.find_entry_index(key);
        match &self.entries[index] {
            Some(existing) => existing == key,
            None => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.iter().all(|item| other.contains(item))
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        self.iter().all(|item| !other.contains(item))
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        self.iter().cloned().chain(other.iter().cloned()).collect()
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        self.iter()
            .filter(|item| other.contains(*item))
            .cloned()
            .collect()
    }

    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        self.iter()
            .filter(|item| !other.contains(*item))
            .cloned()
            .collect()
    }

    fn find_entry_index(&self, key: &T) -> usize {
        if self.capacity == 0 {
            return 0;
        }

        // Pick a slot for the key based on the current capacity
        let hash = hash_item(key);
        let mut index = hash as usize % self.capacity;

        // Linear probing
        // loop until we find an empty slot or an existing key
        loop {
            match &self.entries[index] {
                None => return index,
                Some(existing) if existing == key => return index,
                _ => index = (index + 1) % self.capacity,
            }
        }
    }

    fn adjust_capacity(&mut self, new_capacity: usize) {
        let mut new_entries = Vec::with_capacity(new_capacity);
        // Fill the vector with empty slots
        for _ in 0..new_capacity {
            new_entries.push(None);
        }

        // Push all existing keys into the new vector
        for key in self.entries.drain(..).flatten() {
            let hash = hash_item(&key);
            let mut index = hash as usize % new_capacity;

            // Loop until we find an empty slot
            while new_entries[index].is_some() {
                index = (index + 1) % new_capacity;
            }

            new_entries[index] = Some(key);
        }

        self.entries = new_entries;
        self.capacity = new_capacity;
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.entries.iter().filter_map(|entry| entry.as_ref())
    }
}

impl<T: Hash + Eq + Clone> Clone for CustomSet<T> {
    fn clone(&self) -> Self {
        CustomSet {
            entries: self.entries.clone(),
            count: self.count,
            capacity: self.capacity,
        }
    }
}

impl<T: Hash + Eq + Clone> FromIterator<T> for CustomSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = CustomSet::new(&[]);
        for item in iter {
            set.add(item);
        }
        set
    }
}

impl<T: Hash + Eq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.count != other.count {
            return false;
        }

        self.iter().all(|item| other.contains(item))
    }
}

impl<T: Hash + Eq + Clone> Eq for CustomSet<T> {}

fn hash_item<T: Hash>(item: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}
