use std::ptr;
use std::boxed::Box;

pub struct LRUCache {
    capacity: i32,
    size: i32,
    head: *mut LRU,
    tail: *mut LRU
}

pub struct KeyValue {
    key: i32,
    value: i32,
}

pub struct LRU {
    prev: *mut LRU,
    next: *mut LRU,
    data: KeyValue
}

impl LRU {
    pub fn uninit() -> Self {
        Self {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            data: KeyValue{ key: 0, value: 0}
        }
    }

    pub fn init(key: i32, value: i32) -> Self {
        Self {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            data: KeyValue{ key, value }
        }
    }
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    pub fn new(capacity: i32) -> Self {
        Self {
            capacity,
            size: 0,
            head: Box::into_raw(Box::new(LRU::uninit())),
            tail: Box::into_raw(Box::new(LRU::uninit())),
        }
    }
    
    pub fn get(&mut self, key: i32) -> i32 {
       match self.find(key) {
           Some(node) => {
               self.flush(node);
               unsafe{
                   println!("[Get] value: {}", (&*node).data.value);
                   (&*node).data.value
               }
           },

           None => {
                println!("[Get] value: {}", -1);
               -1
           }
       }
        
    }

    
    pub fn put(&mut self, key: i32, value: i32) {
        // println!("size: {}", self.size);
        match self.find(key) {
            Some(node) => {
                unsafe {
                    let node = &mut *node;
                    // node.data.key = key;
                    node.data.value = value;
                }
            },

            None => {
                // println!("no find");
                let new_node = Box::into_raw(Box::new(LRU::init(key, value)));
                if self.size == 0 {
                    self.head = new_node;
                    self.tail = new_node;
                    self.size += 1;
                }else if self.size < self.capacity {
                    self.append(new_node);
                    self.size += 1;
                }else if self.size == self.capacity {
                    // 此时需要替换LRU最后一个元素
                    // println!("replace");
                    self.replace(new_node);
                }
            }
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
            // if !(&*self.tail).prev.is_null() {
            //     (&mut *(&*self.tail).prev).next = node;
            // }
            (&mut *(&*self.tail).prev).next = node;
            drop(self.tail);
            self.tail = node;
        }
    }

    fn flush(&mut self, node: *mut LRU) {
        if node as *const _ == self.head as *const _ {
            return
        }
        unsafe{
            let data = &mut *node;
            if !data.prev.is_null() {
                (&mut *data.prev).next = data.next;
            }
            if !data.next.is_null() {
                (&mut *data.next).prev = data.prev;
            }
            data.prev = ptr::null_mut();
            data.next = self.head;
            (&mut *self.head).prev = node;
            self.head = node;
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
                // println!("next");
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