
// Block size is 4096 bytes
const BLOCK_SIZE: usize = 500 * 1024;

const MB_SIZE : usize = 1024 * 1024;

const MEMORY_SIZE : usize = 5024 * MB_SIZE;

const BLOCK_NUMS : usize = MEMORY_SIZE/BLOCK_SIZE;

const THREAD_NUMS : usize = 48;

const PER_THREAD_BLOCK_NUMS : usize = BLOCK_NUMS/THREAD_NUMS;

use std::sync::Arc;
use criterion::{Criterion, criterion_group, criterion_main};
use common_cache::{Cache, LruCache};
use parking_lot::RwLock;

fn test_cache_throughput(){
    // Create a Cache<u32,Vec<u8>> with a maximum size of 200mb and a BytesMeter
    let cache = RwLock::new(LruCache::<usize, Arc<Vec<u8>>>::new((MEMORY_SIZE + MB_SIZE) as u64));
    // Create a 4kb block
    let block = vec![0u8; BLOCK_SIZE];
    // Insert 124 mb of blocks into the cache
    for i in 0..BLOCK_NUMS {
        cache.write().put(i, Arc::new(block.clone()));
    }
    // Create a result vector
    let mut result = vec![vec![0u8;BLOCK_SIZE]; BLOCK_NUMS];
    // Get the blocks from the cache, and start to measure the time
    let start = std::time::Instant::now();
    for i in 0..BLOCK_NUMS {
        // Clone for real usage
        let vec = & mut result[i];
        vec.copy_from_slice(cache.write().get(&i).unwrap().as_ref());
    }
    // Measure the time
    let duration = start.elapsed();
    // Print the time
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    // Print the throughput
    println!("Throughput is: {} MB/s", (MEMORY_SIZE/MB_SIZE)as f64 / duration.as_secs_f64());
}

fn test_single_thread_throughput(){
    let mut buffer = vec![0u8;MEMORY_SIZE];
    // write something to the buffer
    let block = vec![1u8; BLOCK_SIZE];
    for i in 0..BLOCK_NUMS {
        let start = i * BLOCK_SIZE;
        let end = start + BLOCK_SIZE;
        buffer[start..end].copy_from_slice(&block);
    }
    let mut result = vec![vec![0u8;BLOCK_SIZE]; BLOCK_NUMS];
    // start
    let start = std::time::Instant::now();
    for i in 0..BLOCK_NUMS {
        let vec = &mut result[i];
        vec.copy_from_slice(&buffer[i*BLOCK_SIZE..(i+1)*BLOCK_SIZE])
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("Throughput is: {} MB/s", (MEMORY_SIZE/MB_SIZE)as f64 / duration.as_secs_f64());
}

// test with multiple threads
fn test_multi_thread_throughput(){
    let mut buffer = vec![0u8;MEMORY_SIZE];
    // write something to the buffer
    let block = vec![1u8; BLOCK_SIZE];
    for i in 0..BLOCK_NUMS {
        let start = i * BLOCK_SIZE;
        let end = start + BLOCK_SIZE;
        buffer[start..end].copy_from_slice(&block);
    }
    // Wrap the buffer into an Arc
    let buffer = Arc::new(buffer);
    let mut handles = vec![];
    for thread_id in 0..THREAD_NUMS {
        let buffer = buffer.clone();
        let handle = std::thread::spawn(move || {
            let mut result = vec![vec![0u8;BLOCK_SIZE]; PER_THREAD_BLOCK_NUMS];
            for i in 0..PER_THREAD_BLOCK_NUMS {
                let vec = & mut result[i];
                let start = thread_id * PER_THREAD_BLOCK_NUMS + i*BLOCK_SIZE;
                let end = start + BLOCK_SIZE;
                vec.copy_from_slice(&buffer[start..end]);
            }
        });
        handles.push(handle);
    }
    // start
    let start = std::time::Instant::now();
    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("Throughput is: {} MB/s", (MEMORY_SIZE/MB_SIZE)as f64 / duration.as_secs_f64());
}


fn criterion_benchmark(c: &mut Criterion) {
    // reduce the sample size to reduce the time
    let mut group = c.benchmark_group("benches");
    group.sample_size(10);
    group.bench_function("test_multi_thread_throughput", |b| b.iter(|| test_multi_thread_throughput()));
    group.bench_function("test_single_thread_throughput", |b| b.iter(|| test_single_thread_throughput()));

    group.bench_function("test_cache_throughput", |b| b.iter(|| test_cache_throughput()));
}
criterion_group!(benches,criterion_benchmark);
// reduce the sample size to reduce the time
criterion_main!(benches);
