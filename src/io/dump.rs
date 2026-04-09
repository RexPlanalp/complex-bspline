pub trait Dump<T> {
    fn dump(&self, obj: &T) -> std::io::Result<()>;
}
