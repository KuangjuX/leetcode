struct LRUCache {
    capacity: i32,
    size: i32,
    head: Option<*mut LRU>,
    tail: Option<*mut LRU>
}

struct KeyValue {
    key: i32,
    value: i32,
}

struct LRU {
    prev: Option<*mut LRU>,
    next: Option<*mut LRU>,
    key: i32,
    value: i32
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            size: 0,
            head: None,
            tail: None
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        let res = self.find(key);
        if res.is_none() {
            return -1
        }
        let node = res.unwrap();
        // 将该节点提取到首位
        let next = node.next;
        let prev = node.prev;

        self.head = Some(node);
        node.next = self.head;
        node.prev = None;
        node.value
    }

    fn find(&mut self, key: i32) -> Option<&mut LRU> {
        let mut node = unsafe {
            &mut *self.head.unwrap()
        };
        while node.next != None {
            if node.key == key {
                return Some(node)
            }
            node = unsafe{ &mut *node.next.unwrap() };
        }
        if node.key == key {
            return Some(node)
        }
        None
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if self.size == 0 {
            let mut node = LRU {
                key,
                value,
                next: None,
                prev: None
            };
            self.head = Some(&mut node);
            self.tail = Some(&mut node);
            self.size += 1;
        }else if self.size < self.capacity {
            let mut node = LRU {
                key,
                value,
                next: None,
                prev: self.tail
            };
            let tail = unsafe{ &mut *self.tail.unwrap() };
            tail.next = Some((&mut node) as *mut LRU);
        }else if self.size == self.capacity {
            let tail = unsafe {
                &mut *self.tail.unwrap()
            };
            tail.key = key;
            tail.value = value;
        }
    }
}

// * Your LRUCache object will be instantiated and called as such:
// * let obj = LRUCache::new(capacity);
// * let ret_1: i32 = obj.get(key);
// * obj.put(key, value);