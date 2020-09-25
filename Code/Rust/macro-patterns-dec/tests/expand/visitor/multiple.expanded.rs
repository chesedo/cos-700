use macro_patterns_dec::visitor;
pub trait Visitor {
    fn visit_input(&mut self, input: &dyn Input) {
        visit_input(self, input)
    }
    fn visit_button(&mut self, button: &dyn Button) {
        visit_button(self, button)
    }
    fn visit_scrollbar(&mut self, scrollbar: &dyn Scrollbar) {
        visit_scrollbar(self, scrollbar)
    }
    fn visit_select_box(&mut self, select_box: &dyn SelectBox) {
        visit_select_box(self, select_box)
    }
    fn visit_radio_button(&mut self, radio_button: &dyn RadioButton) {
        visit_radio_button(self, radio_button)
    }
    fn visit_checkbox(&mut self, checkbox: &dyn Checkbox) {
        visit_checkbox(self, checkbox)
    }
}
pub fn visit_input<V>(_visitor: &mut V, _input: &dyn Input)
where
    V: Visitor + ?Sized,
{
}
pub fn visit_button<V>(_visitor: &mut V, _button: &dyn Button)
where
    V: Visitor + ?Sized,
{
}
pub fn visit_scrollbar<V>(_visitor: &mut V, _scrollbar: &dyn Scrollbar)
where
    V: Visitor + ?Sized,
{
}
pub fn visit_select_box<V>(_visitor: &mut V, _select_box: &dyn SelectBox)
where
    V: Visitor + ?Sized,
{
}
pub fn visit_radio_button<V>(_visitor: &mut V, _radio_button: &dyn RadioButton)
where
    V: Visitor + ?Sized,
{
}
pub fn visit_checkbox<V>(_visitor: &mut V, _checkbox: &dyn Checkbox)
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
impl Visitable for dyn Button {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_button(self);
    }
}
impl Visitable for dyn Scrollbar {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_scrollbar(self);
    }
}
impl Visitable for dyn SelectBox {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_select_box(self);
    }
}
impl Visitable for dyn RadioButton {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_radio_button(self);
    }
}
impl Visitable for dyn Checkbox {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_checkbox(self);
    }
}
