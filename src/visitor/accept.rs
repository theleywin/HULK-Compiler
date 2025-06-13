use super::visitor_trait::Visitor;

pub trait Accept {
    fn accept<V: Visitor<T>,T>(&mut self, visitor: &mut V) -> T;
}
