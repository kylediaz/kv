use std::fmt::Debug;

use crate::ds::list::Deque;
use crate::ds::list::array::ArrayDeque;
use crate::ds::list::dll::DoublyLinkedList;

/// quicklist -- fast dequeue data structure implementation
///
/// A quicklist is an implementation of a linked list dequeue with
/// the same cache-locality optimization technique as a B-tree.
/// I thought of this approach all on my own but then I googled
/// it, it turns out Redis already uses it. Such is life.

pub struct Quicklist<T> {
    inner: DoublyLinkedList<ArrayDeque<T>>,
    len: usize,
}

impl<T> Quicklist<T> {
    pub fn new() -> Self {
        Self {
            inner: DoublyLinkedList::new(),
            len: 0,
        }
    }
}

impl<T> Deque<T> for Quicklist<T> {
    fn len(&self) -> usize {
        self.len
    }

    fn rpush(&mut self, val: T) {
        let tail = match self.inner.rpeek_mut() {
            Some(node) if !node.is_full() => node,
            _ => {
                let new_node = ArrayDeque::empty();
                self.inner.rpush(new_node);
                self.inner.rpeek_mut().unwrap()
            }
        };
        self.len += 1;
        tail.rpush(val)
    }

    fn rpop(&mut self) -> Option<T> {
        let tail = self.inner.rpeek_mut()?;
        let output = tail.rpop();
        if tail.is_empty() {
            self.inner.rpop();
        }
        self.len -= 1;
        output
    }

    fn rpeek(&self) -> Option<&T> {
        let tail = self.inner.rpeek()?;
        tail.rpeek()
    }

    fn rpeek_mut(&mut self) -> Option<&mut T> {
        let tail = self.inner.rpeek_mut()?;
        tail.rpeek_mut()
    }

    fn lpush(&mut self, val: T) {
        let head = match self.inner.lpeek_mut() {
            Some(node) if !node.is_full() => node,
            _ => {
                let new_node = ArrayDeque::empty();
                self.inner.lpush(new_node);
                self.inner.lpeek_mut().unwrap()
            }
        };
        self.len += 1;
        head.lpush(val)
    }

    fn lpop(&mut self) -> Option<T> {
        let head = self.inner.lpeek_mut()?;
        let output = head.lpop();
        if head.is_empty() {
            self.inner.lpop();
        }
        self.len -= 1;
        output
    }

    fn lpeek(&self) -> Option<&T> {
        let head = self.inner.lpeek()?;
        head.lpeek()
    }

    fn lpeek_mut(&mut self) -> Option<&mut T> {
        let head = self.inner.lpeek_mut()?;
        head.lpeek_mut()
    }
}

impl<T> Debug for Quicklist<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
