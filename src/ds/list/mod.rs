mod quicklist;

pub trait Dequeue<T> {
    fn lpush(&mut self, val: T);
    fn rpush(&mut self, val: T);
    fn lpop(&mut self) -> Option<T>;
    fn rpop(&mut self) -> Option<T>;
}

pub use self::quicklist::Quicklist as List;
