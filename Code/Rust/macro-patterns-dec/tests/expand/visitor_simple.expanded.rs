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
