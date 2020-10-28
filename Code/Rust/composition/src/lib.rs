pub trait Button {}
pub trait Scroller {}
pub trait DisplayText {}

struct ConfirmationDialog {
    button: Box<dyn Button>,
    scroller: Box<dyn Scroller>,
    display_text: Box<dyn DisplayText>,
}

impl ConfirmationDialog {
    // Constructor to build a new `ConfirmationDialog`
    pub fn compose(
        button: Box<dyn Button>,
        scroller: Box<dyn Scroller>,
        display_text: Box<dyn DisplayText>,
    ) -> Self {
        ConfirmationDialog {
            button,
            scroller,
            display_text,
        }
    }

    pub fn show(&self) {}
}
