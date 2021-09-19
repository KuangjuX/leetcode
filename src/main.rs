mod system;

use system::LRUCache;

fn main() {
    let mut lru_cache = LRUCache::new(2);
    lru_cache.put(1, 1);
    lru_cache.put(2, 2);
    lru_cache.get(1);
    // println!("put 3, 3");
    lru_cache.put(3, 3);
    // println!("get 2");
    lru_cache.get(2);
    lru_cache.put(4, 4);
    lru_cache.get(1);
    lru_cache.get(3);
    lru_cache.get(4);
}