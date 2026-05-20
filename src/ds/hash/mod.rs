pub trait Map<T> {
    fn set(&mut self, val: T);
    fn get(&mut self) -> Option<T>;
}
