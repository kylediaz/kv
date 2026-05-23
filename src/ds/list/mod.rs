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
    fn test_lpop_empty() {
        let mut list: List<i32> = List::new();
        assert!(list.lpop().is_none())
    }

    #[test]
    fn test_rpop_empty() {
        let mut list: List<i32> = List::new();
        assert!(list.rpop().is_none())
    }

    #[test]
    fn test_lop() {
        let mut list: List<i32> = List::new();
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
        let mut list: List<i32> = List::new();
        list.rpush(1);
        assert_eq!(list.rpop().unwrap(), 1);
        list.rpush(101);
        assert_eq!(list.rpop().unwrap(), 101);
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
            println!("{:?} {}", v, n);
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
            println!("{:?} {}", v, n);
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
            println!("{:?} {}", v, n);
            assert_eq!(v.unwrap(), n);
        }
    }

    /// Tests a large amount of ops that will exceed
    /// page size of hybrid implementations
    #[test]
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
            println!("{:?} {}", v, n);
            assert_eq!(v.unwrap(), n);
        }
    }
}
