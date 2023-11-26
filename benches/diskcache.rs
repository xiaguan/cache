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
    println!("[RocksDB] Write throughput: {} MB/S", throughput);

    let start = std::time::Instant::now();
    for i in 0..BLOCK_NUM {
        assert_eq!(cache.get(INUM, block_ids[i]).await.unwrap().is_some(), true);
    }
    let end = std::time::Instant::now();
    // Calculate the throughput MB/S
    let throughput = (TOTAL_SIZE as f64) / (end - start).as_secs_f64() / 1024.0 / 1024.0;
    println!("[RocksDB] Read throughput: {} MB/S", throughput);
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

// Use criterion to benchmark.
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rocksdb", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(bench_rocksdb())
        })
    });
    c.bench_function("diskcache", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(bench_diskcache())
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
