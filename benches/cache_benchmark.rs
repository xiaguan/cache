use std::sync::Arc;
use criterion::{Criterion, criterion_group, criterion_main};
use hashlink::LruCache;
use common_cache::BasicCache;
use common_cache::fifo::Fifo;

/// Use trait BasicCache to test get and put operations.
/// Key is a i32,value is a i32.

enum Operation{
    Get,
    Put
}

struct Command {
    operation: Operation,
    key: i32,
    value: i32
}

const CACHE_SIZE: usize = 1000;
const COMMANDS: usize = 2000;
fn generate_bench_commands() -> Vec<Command> {
    let mut commands = Vec::new();
    for _ in 0..COMMANDS {
        let operation = if rand::random::<bool>() {
            Operation::Get
        } else {
            Operation::Put
        };
        let key = rand::random::<i32>()  ;
        let value = rand::random::<i32>();
        commands.push(Command {
            operation,
            key,
            value
        });
    }
    commands
}

struct TestCache<C : BasicCache<i32,i32>> {
    cache: C,
    commands: Arc<Vec<Command>>
}

impl <C : BasicCache<i32,i32>> TestCache<C> {
    fn new(cache: C, commands: Arc<Vec<Command>>) -> Self {
        Self {
            cache,
            commands
        }
    }

    fn run(&mut self) {
        self.commands.iter().for_each(|command| {
            match command.operation {
                Operation::Get => {
                    self.cache.get_basic(&command.key);
                }
                Operation::Put => {
                    self.cache.put_basic(command.key, command.value);
                }
            }
        });
    }
}

fn fifo_bench(commands : Arc<Vec<Command>>) {
    let mut test_cache = TestCache::new(Fifo::new(CACHE_SIZE), commands);
    test_cache.run();
}

fn lru_bench(commands : Arc<Vec<Command>>) {
    let mut test_cache = TestCache::new(LruCache::new(CACHE_SIZE), commands);
    test_cache.run();
}

fn cache_bench(c : &mut Criterion) {
    let commands = Arc::new(generate_bench_commands());
    c.bench_function("lru_bench", |b| b.iter(|| lru_bench(commands.clone())));
    c.bench_function("fifo_bench", |b| b.iter(|| fifo_bench(commands.clone())));
}

criterion_group!(benches, cache_bench);
criterion_main!(benches);