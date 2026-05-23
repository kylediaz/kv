use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::ds::list::Deque;

struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    value: T,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

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
        let new_node = Rc::new(RefCell::new(Node {
            $prev: None,
            $next: $self.$head.clone(),
            value: $val,
        }));
        $self.len += 1;
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

        $self.len -= 1;
        match Rc::try_unwrap(head) {
            Ok(node) => Some(node.into_inner().value),
            Err(_) => panic!("Invalid state"),
        }
    }};
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
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
