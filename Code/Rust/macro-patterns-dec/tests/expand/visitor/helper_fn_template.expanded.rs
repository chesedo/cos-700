use macro_patterns_dec::visitor;
pub trait Visitor {
    fn visit_window(&mut self, window: &Window) {
        visit_window(self, window)
    }
    fn visit_group(&mut self, group: &dyn Group) {
        visit_group(self, group)
    }
}
pub fn visit_window<V>(visitor: &mut V, window: &Window)
where
    V: Visitor + ?Sized,
{
    window.get_children().iter().for_each(|child| {
        match child {
            Child::Button(button) => visitor.visit_button(button.deref()),
            Child::Input(input) => visitor.visit_input(input.deref()),
        };
    });
}
pub fn visit_group<V>(visitor: &mut V, group: &dyn Group)
where
    V: Visitor + ?Sized,
{
    group.get_children().iter().for_each(|child| {
        match child {
            Child::Button(button) => visitor.visit_button(button.deref()),
            Child::Input(input) => visitor.visit_input(input.deref()),
        };
    });
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
