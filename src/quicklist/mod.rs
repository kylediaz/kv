use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

/// quicklist -- fast dequeue data structure implementation
///
/// A quicklist is an implementation of a linked list dequeue with
/// the same cache-locality optimization technique as a B-tree.
/// I thought of this approach all on my own but then I googled
/// it and it turns out Redis already uses it.
///

pub trait Dequeue<T> {
    fn lpush(&mut self, val: T);
    fn rpush(&mut self, val: T);
    fn lpop(&mut self) -> Option<T>;
    fn rpop(&mut self) -> Option<T>;
}

struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    value: T,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Quicklist<T> {
    head: Link<T>,
    tail: Link<T>,
}

/**
 * These macros take advantage of the symmetry of doubly linked lists.
 * These functions are implemented from the perspective of left operations.
 */

macro_rules! push_impl {
    ($self:expr, $val:expr, $head:ident, $prev:ident, $next:ident) => {{
        let new_node = Rc::new(RefCell::new(Node {
            $prev: None,
            $next: $self.$head.clone(),
            value: $val,
        }));
        match $self.$head.take() {
            None => {
                $self.head = Some(new_node.clone());
                $self.tail = Some(new_node);
            }
            Some(old_head) => {
                old_head.borrow_mut().$prev = Some(new_node.clone());
                $self.$head = Some(new_node);
            }
        }
    }};
}

macro_rules! pop_impl {
    ($self:expr, $head:ident, $tail:ident, $prev:ident, $next:ident) => {{
        let head = $self.$head.take()?;
        let next = head.borrow_mut().$next.take();

        match next {
            None => $self.$tail = None,
            Some(next) => {
                next.borrow_mut().$prev = None;
                $self.$head = Some(next)
            }
        }

        match Rc::try_unwrap(head) {
            Ok(node) => Some(node.into_inner().value),
            Err(_) => panic!("Invalid state"),
        }
    }};
}

impl<T> Quicklist<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}

impl<T> Dequeue<T> for Quicklist<T> {
    fn rpush(&mut self, val: T) {
        push_impl!(self, val, tail, left, right)
    }

    fn rpop(&mut self) -> Option<T> {
        pop_impl!(self, tail, head, left, right)
    }

    fn lpush(&mut self, val: T) {
        push_impl!(self, val, head, right, left)
    }

    fn lpop(&mut self) -> Option<T> {
        pop_impl!(self, head, tail, right, left)
    }
}

impl<T> Debug for Quicklist<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lpop_empty() {
        let mut list: Quicklist<i32> = Quicklist::new();
        assert!(list.rpop().is_none())
    }

    #[test]
    fn test_rpop_empty() {
        let mut list: Quicklist<i32> = Quicklist::new();
        assert!(list.rpop().is_none())
    }

    #[test]
    fn test_lop() {
        let mut list: Quicklist<i32> = Quicklist::new();
        list.lpush(1);
        assert_eq!(list.lpop().unwrap(), 1);
        assert!(list.lpop().is_none());
        list.lpush(11);
        list.lpush(111);
        assert_eq!(list.lpop().unwrap(), 111);
        assert_eq!(list.lpop().unwrap(), 11);
        assert!(list.lpop().is_none())
    }

    #[test]
    fn test_rop() {
        let mut list: Quicklist<i32> = Quicklist::new();
        list.rpush(1);
        assert_eq!(list.rpop().unwrap(), 1);
        list.rpush(101);
        assert_eq!(list.rpop().unwrap(), 101);
        assert!(list.rpop().is_none())
    }

    #[test]
    fn test_lops() {
        let mut list: Quicklist<i32> = Quicklist::new();
        for n in 1..=100 {
            list.lpush(n);
        }
        for n in (1..=100).rev() {
            let v = list.lpop();
            println!("{:?} {}", v, n);
            assert_eq!(v.unwrap(), n);
        }
    }

    #[test]
    fn test_rops() {
        let mut list: Quicklist<i32> = Quicklist::new();
        for n in 1..=100 {
            list.rpush(n);
        }
        for n in (1..=100).rev() {
            let v = list.rpop();
            println!("{:?} {}", v, n);
            assert_eq!(v.unwrap(), n);
        }
    }

    #[test]
    fn test_lrops() {
        let mut list: Quicklist<i32> = Quicklist::new();
        // This will generate a structure ...7531246...
        for n in 1..=100 {
            if n % 2 == 0 {
                list.rpush(n);
            } else {
                list.lpush(n);
            }
        }
        for n in (1..=100).rev() {
            let v;
            if n % 2 == 0 {
                v = list.rpop();
            } else {
                v = list.lpop();
            }
            println!("{:?} {}", v, n);
            assert_eq!(v.unwrap(), n);
        }
    }
}
