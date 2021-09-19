use std::ptr;

struct LRUCache {
    capacity: i32,
    size: i32,
    head: *mut LRU,
    tail: *mut LRU
}

struct KeyValue {
    key: i32,
    value: i32,
}

struct LRU {
    prev: *mut LRU,
    next: *mut LRU,
    data: KeyValue
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
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
       match self.find(key) {
           Some(node) => {
               self.flush(node);
               unsafe{
                   (&*node).data.value
               }
           },

           None => {
               -1
           }
       }
        
    }

    
    fn put(&mut self, key: i32, value: i32) {
        let mut new_node = LRU{
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            data: KeyValue{ key, value }
        };
        if self.size == 0 {
            self.head = &mut new_node as *mut LRU;
            self.tail = &mut new_node as *mut LRU;
            self.size += 1;
        }else if self.size < self.capacity {
            self.append(&mut new_node as *mut LRU);
            self.size += 1;
        }else if self.size == self.capacity {
            // 此时需要替换LRU最后一个元素
            self.replace(&mut new_node as *mut LRU);
        }
    }

    fn append(&mut self, node: *mut LRU) {
        unsafe{
            (&mut *self.tail).next = node;
            (&mut *node).prev = self.tail;
            (&mut *node).next = ptr::null_mut();
            self.tail = node;
        }
    }

    fn replace(&mut self, node: *mut LRU) {
        unsafe{
            (&mut *node).prev = (&mut *self.tail).prev;
            (&mut *node).next = ptr::null_mut();
            self.tail = node;
        }
    }

    fn flush(&mut self, node: *mut LRU) {
        unsafe{
            let data = &mut *node;
            (&mut *data.prev).next = data.next;
            (&mut *data.next).prev = data.prev;
            data.prev = ptr::null_mut();
            data.next = self.tail;
            self.tail = node;
        }
    }

    fn find(&self, key: i32) -> Option<*mut LRU> {
        unsafe{
            let mut ptr = self.head;
            loop {
                if ptr.is_null() {
                    return None
                }
                let node = &mut *ptr;
                if node.data.key == key {
                    return Some(ptr)
                }
                ptr = node.next;
            }
        }
    }
}

// pub struct IntoIter(LRUCache);

// impl Iterator for IntoIter {
//     type Item = KeyValue;

//     fn next(&mut self) -> Option<KeyValue> {
        
//     }
// }

// * Your LRUCache object will be instantiated and called as such:
// * let obj = LRUCache::new(capacity);
// * let ret_1: i32 = obj.get(key);
// * obj.put(key, value);