use crate::core::knot_vector::KnotVector;

pub trait Dump {
    fn dump<K: KnotVector>(&self, kv: &K) -> std::io::Result<()>;
}