use super::visitor_trait::Visitor;

pub trait Accept {
    fn accept<V: Visitor<T>,T>(&self, visitor: &mut V) -> T;
}
