use crate::ds::list::Deque;

/// An array-based deque implementation with fixed capacity.
/// Meant to be used as part of hybrid data structures.

const PAGE_SIZE: usize = 4096;

/// Classic array deque implementation
///
/// Capacity = PAGE_SIZE - 1
/// Invariants:
/// - 0 <= l, r < PAGE_SIZE
/// - if l <= r: values[i].is_some() on [l, r)
/// - if r < l:  values[i].is_some() on [0, r), [l, PAGE_SIZE)
pub struct ArrayDeque<T> {
    values: [Option<T>; PAGE_SIZE],
    l: usize,
    r: usize,
}

macro_rules! wrapping_add {
    ($value:expr, $diff:expr) => {
        ($value + $diff) % PAGE_SIZE
    };
}
macro_rules! wrapping_dec {
    ($value:expr, $diff:expr) => {
        (PAGE_SIZE + $value - $diff) % PAGE_SIZE
    };
}

impl<T> ArrayDeque<T> {
    pub fn empty() -> Self {
        ArrayDeque {
            values: [const { None }; PAGE_SIZE],
            l: 0,
            r: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.len() == PAGE_SIZE - 1
    }
}

impl<T> Deque<T> for ArrayDeque<T> {
    fn len(&self) -> usize {
        (PAGE_SIZE + self.r - self.l) % PAGE_SIZE
    }
    fn rpush(&mut self, val: T) {
        assert!(!self.is_full());
        let old = self.values[self.r].replace(val);
        self.r = wrapping_add!(self.r, 1);
        assert!(old.is_none());
    }

    fn rpop(&mut self) -> Option<T> {
        assert!(!self.is_empty());
        self.r = wrapping_dec!(self.r, 1);
        let output = self.values[self.r].take();
        assert!(output.is_some());
        output
    }

    fn rpeek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.values[wrapping_dec!(self.r, 1)].as_ref()
        }
    }

    fn rpeek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.values[wrapping_dec!(self.r, 1)].as_mut()
        }
    }

    fn lpush(&mut self, val: T) {
        assert!(!self.is_full());
        self.l = wrapping_dec!(self.l, 1);
        let old = self.values[self.l].replace(val);
        assert!(old.is_none());
    }

    fn lpop(&mut self) -> Option<T> {
        assert!(!self.is_empty());
        let output = self.values[self.l].take();
        self.l = wrapping_add!(self.l, 1);
        assert!(output.is_some());
        output
    }

    fn lpeek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.values[self.l].as_ref()
        }
    }

    fn lpeek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.values[self.l].as_mut()
        }
    }
}
