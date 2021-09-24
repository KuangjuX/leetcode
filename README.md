# leetcode-rs

My solution for leetcode in Rust.

## 链表
- [x] [19 删除链表倒数第n个节点](链表/19-删除链表倒数第n个节点.rs)  
- [x] [92 反转链表 II](链表/92-反转链表II.rs)  

使用 Rust 写链表题是非常痛苦的，因为 Rust 的所有权机制最多允许多个不可变引用或者一个可变引用对于一个结点引用，而对于链表来说，一个结点上有多个引用是很正常的事情，在对于链表进行操作时，如果我们不需要对链表进行修改的话，我们只需要拥有链表的引用即可，如果我们需要重新构造链表的话，则必须使用 `take()` 方法抢夺节点的所有权来重新构造链表。   
   
在 **第92题 反转链表 II** 的时候，我的做法是遍历所有链表的节点，获取其值放入 `Vec` 中，并对其进行反转操作，随后重新构造链表进行反转：
```rust
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let left = left as usize;
        let right = right as usize;
        let mut cur = head;
        let mut v = vec![];
        let mut root = Box::new(ListNode::new(0));
        let mut node = &mut root;

        loop {
            if let Some(node) = cur {
                cur = node.next;
                v.push(node.val);
            }else {
                break;
            }
        }
        let mid = unsafe {
            slice::from_raw_parts_mut(
                v.as_mut_ptr().offset((left - 1) as isize), 
                right - left + 1
            )
        };
        mid.reverse();

        for i in 0..v.len() {
            if i < left - 1 || i > right - 1 {
                let next = Box::new(ListNode::new(v[i]));
                node.next = Some(next);
                node = node.next.as_mut().unwrap();
            }else {
                let index = i - left + 1;
                let next = Box::new(ListNode::new(mid[index]));
                node.next = Some(next);
                node = node.next.as_mut().unwrap();
            }
        }

        root.next
    }
```
而车春池学长的做法很巧妙，他获取节点的所有权并打乱节点的顺序，重新构造了链表，和我的解法相比空间复杂度大大地降低了：
```rust
    pub fn reverse_between2(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(0));
        let mut pos = root.as_mut();
        let mut counter = 1;
        let mut stack = Vec::new();
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if counter < left {
                pos = pos.next.get_or_insert(node);
                counter += 1;
            } else if counter <= right {
                stack.push(node);
                if counter == right {
                    break;
                } else {
                    counter += 1;
                }
            } else {
                unreachable!()
            }
        }
        while let Some(node) = stack.pop() {
            pos = pos.next.get_or_insert(node);
        }
        while let Some(mut node) = head.take() {
            head = node.next.take();
            pos = pos.next.get_or_insert(node);
        }
        
        root.next
    }
```