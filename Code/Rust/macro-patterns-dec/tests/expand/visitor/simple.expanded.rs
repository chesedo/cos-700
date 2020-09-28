use macro_patterns_dec::visitor;
pub trait Visitor {
    fn visit_input(&mut self, input: &dyn Input) {
        visit_input(self, input)
    }
}
pub fn visit_input<V>(_visitor: &mut V, _input: &dyn Input)
where
    V: Visitor + ?Sized,
{
}
trait Visitable {
    fn apply(&self, visitor: &mut dyn Visitor);
}
impl Visitable for dyn Input {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_input(self);
    }
}
