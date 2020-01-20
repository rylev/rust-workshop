use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// How many buckets the hashmap begins with
const INITIAL_BUCKET_COUNT: usize = 16;
// The max load factor as computed by the average number of items per slot
const MAX_LOAD_FACTOR: f64 = 0.9;

type Slot<K, V> = Option<((K, V), usize)>;
pub struct HashMap<K: Hash + Eq, V> {
    slots: Vec<Slot<K, V>>,
    slot_count: usize,
    item_count: usize,
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        HashMap {
            slots: Self::create_slots(INITIAL_BUCKET_COUNT),
            slot_count: INITIAL_BUCKET_COUNT,
            item_count: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let load_factor = self.item_count as f64 / self.slot_count as f64;
        if load_factor >= MAX_LOAD_FACTOR {
            // The load factor is higher than what we want. We must resize.
            self.resize();
        }

        let new_slot_index = self.slot_index(&key);
        let slot = self.slot_mut(new_slot_index, &key);
        match slot {
            Some(slot) => {
                let old = slot.replace(((key, value), new_slot_index));

                match old {
                    Some(((_, v), _)) => Some(v),
                    None => {
                        self.item_count += 1;
                        None
                    }
                }
            }
            None => {
                self.slots.push(Some(((key, value), new_slot_index)));
                None
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let slot_index = self.slot_index(key);
        let slot = self.slot(slot_index, key)?;
        let ((_, v), _) = slot.as_ref()?;
        Some(v)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let slot_index = self.slot_index(&key);
        let slot = self.slot_mut(slot_index, key)?;
        let ((_, v), _) = slot.take()?;
        Some(v)
    }

    fn resize(&mut self) {
        self.slot_count *= 2;
        let new_slots = Self::create_slots(self.slot_count);

        let old_slots = std::mem::replace(&mut self.slots, new_slots);
        for old_slot in old_slots.into_iter() {
            if let Some(((key, value), slot_index)) = old_slot {
                let slot = self.slot_mut(slot_index, &key).unwrap();
                *slot = Some(((key, value), slot_index));
            }
        }
    }

    fn slot_mut(&mut self, slot_index: usize, key: &K) -> Option<&mut Slot<K, V>> {
        self.slots
            .iter_mut()
            .skip(slot_index)
            .find(|item| match item {
                Some(((k, _), _)) => k == key,
                None => true,
            })
    }

    fn slot(&self, slot_index: usize, key: &K) -> Option<&Slot<K, V>> {
        self.slots.iter().skip(slot_index).find(|item| match item {
            Some(((k, _), _)) => k == key,
            None => true,
        })
    }

    fn slot_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % self.slot_count as u64) as usize
    }

    fn create_slots(slot_count: usize) -> Vec<Slot<K, V>> {
        let mut new_slots = Vec::with_capacity(slot_count);
        for _ in 0..slot_count {
            new_slots.push(None);
        }
        new_slots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_works() {
        let mut map = HashMap::new();
        assert_eq!(map.insert("foo", "bar"), None);
        assert_eq!(map.insert("foo", "lol"), Some("bar"));

        assert_eq!(map.get(&"foo"), Some(&"lol"));
        assert_eq!(map.get(&"foo"), Some(&"lol"));
        assert_eq!(map.get(&"qux"), None);

        assert_eq!(map.remove(&"foo"), Some("lol"));
        assert_eq!(map.get(&"foo"), None);
    }
}
