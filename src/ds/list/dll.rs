#![allow(dead_code)]

use std::fmt::Debug;
use std::ptr;

use crate::ds::list::Deque;

struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    value: T,
}

type Link<T> = *mut Node<T>;

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

// Safety: No method leaks internal Rc/Refcell references, therefore
//   there can't be cross-thread aliasing of those Rcs.
unsafe impl<T> Send for DoublyLinkedList<T> where T: Send {}

/**
 * These macros take advantage of the symmetry of doubly linked lists.
 * These functions are implemented from the perspective of left operations.
 */

macro_rules! push_impl {
    ($self:expr, $val:expr, $head:ident, $prev:ident, $next:ident) => {{
        unsafe {
            let new_node = Box::into_raw(Box::new(Node {
                $prev: ptr::null_mut(),
                $next: $self.$head,
                value: $val,
            }));
            $self.len += 1;
            if $self.$head.is_null() {
                $self.head = new_node;
                $self.tail = new_node;
            } else {
                let old_head = $self.$head;
                (*old_head).$prev = new_node;
                $self.$head = new_node;
            }
        }
    }};
}

macro_rules! pop_impl {
    ($self:expr, $head:ident, $tail:ident, $prev:ident, $next:ident) => {{
        unsafe {
            if $self.$head.is_null() {
                None
            } else {
                let old_head = Box::from_raw($self.$head);
                $self.$head = old_head.$next;
                if $self.$head.is_null() {
                    $self.$tail = ptr::null_mut();
                } else {
                    (*$self.$head).$prev = ptr::null_mut();
                };
                $self.len -= 1;
                Some(old_head.value)
            }
        }
    }};
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }
}

impl<T> Deque<T> for DoublyLinkedList<T> {
    fn len(&self) -> usize {
        self.len
    }

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

impl<T> Debug for DoublyLinkedList<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.rpop() {}
    }
}
