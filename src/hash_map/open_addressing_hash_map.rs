use std::{
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

struct HashMap<K: Hash + Sized + Eq, V: Clone> {
    entries: Vec<Option<Entry<K, V>>>,
    len: usize,
}

impl<K: Hash + Sized + Eq, V: Clone> HashMap<K, V> {
    fn new() -> Self {
        Self::with_capacity(INIT_CAPACITY)
    }
    fn with_capacity(cap: usize) -> Self {
        let mut entries = Vec::with_capacity(cap);
        entries.resize_with(cap, || None);
        Self { entries, len: 0 }
    }

    fn insert(&mut self, key: K, value: V) {
        if self.len == self.entries.capacity() {
            self.resize();
        }

        let mut index = self.hash(&key);
        if self.entries[index].is_none() {
            self.entries[index] = Some(Entry::new(key, value));
            self.len += 1;
            return;
        }
        while self.entries[index].as_ref().unwrap().key != key {
            index = (index + 1) % self.entries.capacity();
        }

        self.entries[index] = Some(Entry::new(key, value));
        self.len += 1;
    }

    fn get(&self, key: &K) -> Option<V> {
        let index = self.hash(key);

        for _ in 0..self.len {
            if let Some(entry) = self.entries[index].as_ref() {
                if *key == entry.key {
                    return Some(entry.value.clone());
                }
            }
        }

        None
    }

    fn exists(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.entries.capacity()
    }

    fn _hash(&self, key: &K) -> usize {
        let bytes: &[u8] = unsafe {
            let ptr: *const u8 = key as *const K as *const u8;
            std::slice::from_raw_parts(ptr, mem::size_of::<K>())
        };
        let mut hash_value: usize = 0;
        for b in bytes.iter() {
            hash_value = (hash_value << 3) + (*b as usize);
        }

        hash_value % self.entries.capacity()
    }

    fn resize(&mut self) {
        let new_cap = self.entries.capacity() * 2;
        self.entries.resize_with(new_cap, || None);
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
