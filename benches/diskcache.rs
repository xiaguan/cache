use std::{fs::File, io::Write, time::Instant};

use common_cache::diskcache::{Block, DiskCache};
use criterion::{criterion_group, criterion_main, Criterion};

// Set inum = 1 , block_ids = [1..250] ,block is 4MB.
// So total size is 1GB.
const INUM: u64 = 1;
const BLOCK_SIZE: usize = 4 * 1024 * 1024;
const BLOCK_NUM: usize = 250;
const TOTAL_SIZE: usize = BLOCK_SIZE * BLOCK_NUM;

async fn bench_diskcache() {
    let tempdir = tempfile::tempdir().unwrap();
    let cache = DiskCache::open(tempdir).await.unwrap();

    let mut block_ids = Vec::new();
    for i in 0..BLOCK_NUM {
        block_ids.push(i as u64);
    }

    let block = Block::from(vec![0; BLOCK_SIZE]);

    let start = std::time::Instant::now();
    for i in 0..BLOCK_NUM {
        cache.set(INUM, block_ids[i], &block).await.unwrap();
    }
    let end = std::time::Instant::now();
    // Calculate the throughput MB/S
    let throughput = (TOTAL_SIZE as f64) / (end - start).as_secs_f64() / 1024.0 / 1024.0;
    println!("[Diskcache] Write throughput: {} MB/S", throughput);

    let start = std::time::Instant::now();
    for i in 0..BLOCK_NUM {
        assert_eq!(cache.get(INUM, block_ids[i]).await.unwrap().is_some(), true);
    }
    let end = std::time::Instant::now();
    // Calculate the throughput MB/S
    let throughput = (TOTAL_SIZE as f64) / (end - start).as_secs_f64() / 1024.0 / 1024.0;
    println!("[Diskcache] Read throughput: {} MB/S", throughput);
}

async fn bench_rocksdb() {
    let tempdir = tempfile::tempdir().unwrap();
    let db = rocksdb::DB::open_default(tempdir.path()).unwrap();

    let mut block_ids = Vec::new();
    for i in 0..BLOCK_NUM {
        block_ids.push(i as u64);
    }

    let block = Block::from(vec![0; BLOCK_SIZE]);

    let start = std::time::Instant::now();
    for i in 0..BLOCK_NUM {
        db.put(block_ids[i].to_be_bytes(), &block.get_data())
            .unwrap();
    }
    db.flush().unwrap();
    let end = std::time::Instant::now();
    // Calculate the throughput MB/S
    let throughput = (TOTAL_SIZE as f64) / (end - start).as_secs_f64() / 1024.0 / 1024.0;
    println!("Write throughput: {} MB/S", throughput);

    let start = std::time::Instant::now();
    for i in 0..BLOCK_NUM {
        assert_eq!(db.get(block_ids[i].to_be_bytes()).unwrap().is_some(), true);
    }
    let end = std::time::Instant::now();
    // Calculate the throughput MB/S
    let throughput = (TOTAL_SIZE as f64) / (end - start).as_secs_f64() / 1024.0 / 1024.0;
    println!("Read throughput: {} MB/S", throughput);
}

fn bench_ssd_seq_write() {
    let file_size_mb = 1024; // The size of the file to be written, in megabytes
    let block_size = 4 * 1_024 * 1_024; // Block size (1 MB)
    let total_size = file_size_mb * block_size;
    let data = vec![0u8; block_size]; // Create a buffer with the block size

    let mut file = File::create("/tmp/test_file.bin").unwrap();

    let start = Instant::now();
    let mut written = 0;

    while written < total_size {
        file.write_all(&data).unwrap();
        written += data.len();
    }

    let duration = start.elapsed();
    let speed_mbps = (written as f64 / 1_024.0 / 1_024.0) / duration.as_secs_f64();

    println!("Sequential Write Speed: {:.2} MB/s", speed_mbps);
}

// Use criterion to benchmark.
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("diskcache", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(bench_diskcache())
        })
    });
    c.bench_function("rocksdb", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(bench_rocksdb())
        })
    });
    c.bench_function("ssd_seq_write", |b| b.iter(|| bench_ssd_seq_write()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
