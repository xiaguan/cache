use crate::BasicCache;
use crossbeam_queue::ArrayQueue;
use std::hash::Hash;
use std::rc::Rc;

pub struct Fifo<K: Eq + Hash, V> {
    queue: ArrayQueue<Rc<K>>,
    table: hashbrown::HashMap<Rc<K>, V>,
    max_capacity: usize,
}

impl<K: Eq + Hash, V> Fifo<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: ArrayQueue::new(capacity),
            table: hashbrown::HashMap::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let key_rc = Rc::new(key);
        if self.table.len() == self.max_capacity && !self.table.contains_key(&key_rc) {
            // Evict only if new key and cache is full
            self.evict();
        }
        self.queue.push(key_rc.clone()).ok()?;
        self.table.insert(key_rc, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.table.get(key)
    }

    fn evict(&mut self) {
        if let Some(key_rc) = self.queue.pop() {
            self.table.remove(&key_rc);
        } else {
            panic!("Queue is empty, but eviction was attempted");
        }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}

impl<K: Eq + Hash, V> BasicCache<K, V> for Fifo<K, V> {
    fn get_basic(&mut self, key: &K) -> Option<&V> {
        Fifo::get(self, key)
    }

    fn put_basic(&mut self, key: K, value: V) -> () {
        Fifo::put(self, key, value);
    }
}
