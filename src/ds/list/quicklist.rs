use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::ds::list::Dequeue;
use crate::ds::list::array::ArrayDeque;

/// quicklist -- fast dequeue data structure implementation
///
/// A quicklist is an implementation of a linked list dequeue with
/// the same cache-locality optimization technique as a B-tree.
/// I thought of this approach all on my own but then I googled
/// it, it turns out Redis already uses it. Such is life.

struct Node<T> {
    prev: Option<Link<T>>,
    next: Option<Link<T>>,
    deque: ArrayDeque<T>,
}

impl<T> Node<T> {
    fn empty() -> Self {
        Node {
            prev: None,
            next: None,
            deque: ArrayDeque::empty(),
        }
    }

    fn new(initial_value: T, prev: Option<Link<T>>, next: Option<Link<T>>) -> Self {
        let mut output = Node {
            prev: prev,
            next: next,
            deque: ArrayDeque::empty(),
        };
        output.deque.rpush(initial_value);
        output
    }
}

type Link<T> = Rc<RefCell<Node<T>>>;

/// Invariants:
/// - For each node:
///     - Is non empty (l < r). If it is empty, it should be removed from the list.
/// - Same invariants as any linked list. e.g.
///     - head.is_none() iff tail.is_none()
///     - head.prev = None, tail.next = None
///     - etc
pub struct Quicklist<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

// Safety: No method leaks internal Rc/Refcell references, therefore
//   there can't be cross-thread aliasing of those Rcs.
unsafe impl<T> Send for Quicklist<T> where T: Send {}

impl<T> Quicklist<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn node_rpush(&mut self) -> Link<T> {
        let new_node = Rc::new(RefCell::new(Node::empty()));
        assert_eq!(self.tail.is_none(), self.head.is_none());
        if self.tail.is_none() && self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            let old_tail = self.tail.replace(new_node.clone()).unwrap();
            old_tail.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(old_tail);
        }
        new_node
    }

    fn node_rpop(&mut self) -> Option<Node<T>> {
        let tail_rc = self.tail.take()?;
        if Rc::ptr_eq(self.head.as_ref().unwrap(), &tail_rc) {
            self.head = None;
            self.tail = None;
        } else {
            self.tail = tail_rc.borrow_mut().prev.take();
            self.tail.as_mut().unwrap().borrow_mut().next = None;
        };
        let node = match Rc::try_unwrap(tail_rc) {
            Ok(cell) => cell.into_inner(),
            Err(_) => panic!("Unreachable"),
        };
        Some(node)
    }

    fn node_lpush(&mut self) -> Link<T> {
        let new_node = Rc::new(RefCell::new(Node::empty()));
        assert_eq!(self.tail.is_none(), self.head.is_none());
        if self.tail.is_none() && self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            let old_head = self.head.replace(new_node.clone()).unwrap();
            old_head.borrow_mut().prev = Some(new_node.clone());
            new_node.borrow_mut().next = Some(old_head);
        }
        new_node
    }

    fn node_lpop(&mut self) -> Option<Node<T>> {
        let head_rc = self.head.take()?;
        if Rc::ptr_eq(self.tail.as_ref().unwrap(), &head_rc) {
            self.head = None;
            self.tail = None;
        } else {
            self.head = head_rc.borrow_mut().next.take();
            self.head.as_mut().unwrap().borrow_mut().prev = None;
        };
        let node = match Rc::try_unwrap(head_rc) {
            Ok(cell) => cell.into_inner(),
            Err(_) => panic!("Unreachable"),
        };
        Some(node)
    }
}

impl<T> Dequeue<T> for Quicklist<T> {
    fn rpush(&mut self, val: T) {
        let tail = match self.tail.clone() {
            Some(tail) => tail,
            None => self.node_rpush(),
        };
        if tail.borrow_mut().deque.is_full() {
            let new_node_link = self.node_rpush();
            new_node_link.borrow_mut().deque.rpush(val);
        } else {
            tail.borrow_mut().deque.rpush(val);
        };
    }

    fn rpop(&mut self) -> Option<T> {
        let mut tail = self.tail.as_mut()?.borrow_mut();
        let output = tail.deque.rpop();
        if tail.deque.is_empty() {
            drop(tail);
            self.node_rpop();
        };
        output
    }

    fn lpush(&mut self, val: T) {
        let head = match self.head.clone() {
            Some(head) => head,
            None => self.node_rpush(),
        };
        if head.borrow_mut().deque.is_full() {
            let new_node_link = self.node_lpush();
            new_node_link.borrow_mut().deque.lpush(val);
        } else {
            head.borrow_mut().deque.lpush(val);
        };
    }

    fn lpop(&mut self) -> Option<T> {
        let mut head = self.head.as_mut()?.borrow_mut();
        let output = head.deque.lpop();
        if head.deque.is_empty() {
            drop(head);
            self.node_lpop();
        };
        output
    }
}

impl<T> Debug for Quicklist<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
