mod array;
mod dll;
mod quicklist;

pub trait Deque<T> {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn lpush(&mut self, val: T);
    fn rpush(&mut self, val: T);
    fn lpop(&mut self) -> Option<T>;
    fn rpop(&mut self) -> Option<T>;

    fn lpeek(&self) -> Option<&T>;
    fn lpeek_mut(&mut self) -> Option<&mut T>;
    fn rpeek(&self) -> Option<&T>;
    fn rpeek_mut(&mut self) -> Option<&mut T>;
}

pub use self::quicklist::Quicklist as List;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_len_empty() {
        let list: List<i32> = List::new();
        assert!(list.len() == 0);
        assert!(list.is_empty());
    }

    #[test]
    fn test_peek_empty() {
        let mut list: List<i32> = List::new();
        assert!(list.lpeek().is_none());
        assert!(list.rpeek().is_none());
        assert!(list.lpeek_mut().is_none());
        assert!(list.rpeek_mut().is_none());
    }

    #[test]
    fn test_lpop_empty() {
        let mut list: List<i32> = List::new();
        assert!(list.lpop().is_none());
        assert!(list.is_empty());
    }

    #[test]
    fn test_rpop_empty() {
        let mut list: List<i32> = List::new();
        assert!(list.rpop().is_none());
        assert!(list.is_empty());
    }

    #[test]
    fn test_lop() {
        let mut list: List<i32> = List::new();
        list.lpush(1);
        assert_eq!(list.lpeek(), Some(&1));
        assert_eq!(list.lpop().unwrap(), 1);
        assert_eq!(list.lpeek(), None);
        assert!(list.lpop().is_none());
        list.lpush(11);
        assert_eq!(list.lpeek(), Some(&11));
        list.lpush(111);
        assert_eq!(list.lpeek(), Some(&111));
        assert_eq!(list.lpop().unwrap(), 111);
        assert_eq!(list.lpop().unwrap(), 11);
        assert!(list.lpop().is_none())
    }

    #[test]
    fn test_rop() {
        let mut list: List<i32> = List::new();
        list.rpush(1);
        assert_eq!(list.rpeek(), Some(&1));
        assert_eq!(list.rpop().unwrap(), 1);
        assert_eq!(list.rpeek(), None);
        assert!(list.rpop().is_none());
        list.rpush(11);
        assert_eq!(list.rpeek(), Some(&11));
        list.rpush(111);
        assert_eq!(list.rpeek(), Some(&111));
        assert_eq!(list.rpop().unwrap(), 111);
        assert_eq!(list.rpop().unwrap(), 11);
        assert!(list.rpop().is_none())
    }

    #[test]
    fn test_lops() {
        let mut list: List<i32> = List::new();
        for n in 1..=100 {
            list.lpush(n);
        }
        for n in (1..=100).rev() {
            let v = list.lpop();
            assert_eq!(v.unwrap(), n);
        }
    }

    #[test]
    fn test_rops() {
        let mut list: List<i32> = List::new();
        for n in 1..=100 {
            list.rpush(n);
        }
        for n in (1..=100).rev() {
            let v = list.rpop();
            assert_eq!(v.unwrap(), n);
        }
    }

    #[test]
    fn test_lrops() {
        let mut list: List<i32> = List::new();
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
            assert_eq!(v.unwrap(), n);
        }
    }

    /// Tests a large amount of ops that will exceed
    /// page size of hybrid implementations
    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_lrops_big() {
        let mut list: List<i32> = List::new();
        // This will generate a structure ...7531246...
        for n in 1..=100_000 {
            if n % 2 == 0 {
                list.rpush(n);
            } else {
                list.lpush(n);
            }
        }
        for n in (1..=100_000).rev() {
            let v;
            if n % 2 == 0 {
                v = list.rpop();
            } else {
                v = list.lpop();
            }
            assert_eq!(v.unwrap(), n);
        }
    }
}
