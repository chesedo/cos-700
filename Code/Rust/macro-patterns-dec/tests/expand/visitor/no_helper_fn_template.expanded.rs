use macro_patterns_dec::visitor;
pub trait Visitor {
    fn visit_window(&mut self, window: &Window);
    fn visit_group(&mut self, group: &dyn Group);
}
trait Visitable {
    fn apply(&self, visitor: &mut dyn Visitor);
}
impl Visitable for Window {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_window(self);
    }
}
impl Visitable for dyn Group {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_group(self);
    }
}
