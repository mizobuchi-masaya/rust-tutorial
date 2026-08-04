fn main() {
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }
}
