use crate::BasicCache;
use std::collections::VecDeque;
use std::hash::Hash;
use std::rc::Rc;

pub struct Fifo<K: Eq + Hash, V> {
    queue: VecDeque<Rc<K>>,
    table: hashbrown::HashMap<Rc<K>, V>,
    max_capacity: usize,
}

impl<K: Eq + Hash, V> Fifo<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            table: hashbrown::HashMap::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if self.table.len() >= self.max_capacity {
            self.evict();
        }
        let key = Rc::new(key);
        self.queue.push_back(key.clone());
        self.table.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.table.get(key)
    }

    fn evict(&mut self) -> Option<(Rc<K>, V)> {
        let key = self.queue.pop_front()?;
        let value = self.table.remove(&key)?;
        Some((key, value))
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
