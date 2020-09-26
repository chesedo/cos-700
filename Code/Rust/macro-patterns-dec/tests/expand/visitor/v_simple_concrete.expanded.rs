use macro_patterns_dec::visitor;
pub trait Visitor {
    fn visit_button(&mut self, button: &Button) {
        visit_button(self, button)
    }
}
pub fn visit_button<V>(_visitor: &mut V, _button: &Button)
where
    V: Visitor + ?Sized,
{
}
trait Visitable {
    fn apply(&self, visitor: &mut dyn Visitor);
}
impl Visitable for Button {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_button(self);
    }
}
