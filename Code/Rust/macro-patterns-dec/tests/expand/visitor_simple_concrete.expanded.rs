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
