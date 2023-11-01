use common_cache::fifo::Fifo;

#[test]
fn test_pub_and_get()
{
    let mut cache = Fifo::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.get(&1), Some(&10));
    assert_eq!(cache.get(&2), Some(&20));
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.len(), 2);
}