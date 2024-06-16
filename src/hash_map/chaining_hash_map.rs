use std::{
    collections::LinkedList,
    hash::{DefaultHasher, Hash, Hasher},
    mem,
};

const INIT_CAPACITY: usize = 2;

struct Entry<K: Hash + Sized + Eq, V: Clone> {
    key: K,
    value: V,
}

impl<K: Hash + Sized + Eq, V: Clone> Entry<K, V> {
    fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}
struct Bucket<K: Hash + Sized + Eq, V: Clone> {
    entries: LinkedList<Entry<K, V>>,
}
impl<K: Hash + Sized + Eq, V: Clone> Bucket<K, V> {
    fn new() -> Self {
        Self {
            entries: LinkedList::new(),
        }
    }
    fn push_front(&mut self, key: K, value: V) {
        self.entries.push_front(Entry::new(key, value));
    }

    fn get(&self, key: &K) -> Option<V> {
        for entry in &self.entries {
            if *key == entry.key {
                return Some(entry.value.clone());
            }
        }
        None
    }
}

struct HashMap<K: Hash + Sized + Eq, V: Clone> {
    buckets: Vec<Option<Bucket<K, V>>>,
    len: usize,
}

impl<K: Hash + Sized + Eq, V: Clone> HashMap<K, V> {
    fn new() -> Self {
        Self::with_capacity(INIT_CAPACITY)
    }

    fn with_capacity(cap: usize) -> Self {
        let mut buckets = Vec::with_capacity(cap);
        buckets.resize_with(cap, || None);
        Self { buckets, len: 0 }
    }

    fn insert(&mut self, key: K, value: V) {
        if self.len == self.buckets.capacity() {
            self.resize();
        }

        let index = self.hash(&key);
        if self.buckets[index].is_none() {
            self.buckets[index] = Some(Bucket::new());
        }

        let bucket = self.buckets[index].as_mut().unwrap();
        bucket.push_front(key, value);
        self.len += 1;
    }

    fn get(&self, key: &K) -> Option<V> {
        let index = self.hash(&key);
        match self.buckets[index].as_ref() {
            None => None,
            Some(bucket) => bucket.get(key),
        }
    }

    fn exists(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.capacity()
        // unsafe {
        //     let ptr: *const u8 = key as *const K as *const u8;
        //     std::slice::from_raw_parts(key, mem::size_of::<K>());
        // }
    }

    fn resize(&mut self) {
        let new_cap = self.buckets.capacity() * 2;
        self.buckets.resize_with(new_cap, || None);
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn test() {
        let mut mp = HashMap::with_capacity(3);
        mp.insert("aaa", "abc");
        mp.insert("bbs", "bbb");

        let a = mp.get(&"aaa").unwrap();
        let b = mp.get(&"bbs").unwrap();
        assert_eq!(a, "abc");
        assert_eq!(b, "bbb");
    }
}
