use common_cache::fifo::Fifo;
use common_cache::BasicCache;
use criterion::{criterion_group, criterion_main, Criterion};
use hashlink::LruCache;
use std::sync::Arc;

/// Use trait BasicCache to test get and put operations.
/// Key is a i32,value is a i32.

type KeyType = i32;
type ValueType = i32;

enum Command {
    Get(KeyType),
    Put(KeyType, ValueType),
}

const CACHE_SIZE: usize = 1000;
const COMMAND_SIZE: usize = 100000;
const RAND_RANGE: usize = 1000;

fn generate_bench_commands() -> Vec<Command> {
    let mut commands = Vec::new();
    for _ in 0..COMMAND_SIZE {
        let command = if rand::random::<bool>() {
            Command::Get(rand::random::<KeyType>() % RAND_RANGE as KeyType)
        } else {
            Command::Put(
                rand::random::<KeyType>() % RAND_RANGE as KeyType,
                rand::random::<ValueType>(),
            )
        };
        commands.push(command);
    }
    commands
}

struct TestCache<C: BasicCache<KeyType, ValueType>> {
    cache: C,
    commands: Arc<Vec<Command>>,
}

impl<C: BasicCache<KeyType, ValueType>> TestCache<C> {
    fn new(cache: C, commands: Arc<Vec<Command>>) -> Self {
        Self { cache, commands }
    }

    fn run(&mut self) {
        for command in self.commands.iter() {
            match command {
                Command::Get(key) => {
                    self.cache.get_basic(key);
                }
                Command::Put(key, value) => {
                    self.cache.put_basic(key.clone(), value.clone());
                }
            }
        }
    }
}

fn fifo_bench(commands: Arc<Vec<Command>>) {
    let mut test_cache = TestCache::new(Fifo::new(CACHE_SIZE), commands);
    test_cache.run();
}

fn lru_bench(commands: Arc<Vec<Command>>) {
    let mut test_cache = TestCache::new(LruCache::new(CACHE_SIZE), commands);
    test_cache.run();
}

fn cache_bench(c: &mut Criterion) {
    let commands = Arc::new(generate_bench_commands());
    c.bench_function("lru_bench", |b| b.iter(|| lru_bench(commands.clone())));
    c.bench_function("fifo_bench", |b| b.iter(|| fifo_bench(commands.clone())));
}

criterion_group!(benches, cache_bench);
criterion_main!(benches);
