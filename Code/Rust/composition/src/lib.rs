pub trait Button {
    fn draw(&self); // Button to draw itself
}
pub trait DisplayText {}

struct ConfirmationDialog {
    button: Box<dyn Button>,
    display_text: Box<dyn DisplayText>,
}

impl ConfirmationDialog {
    // Constructor to build a new `ConfirmationDialog`
    pub fn compose(button: Box<dyn Button>, display_text: Box<dyn DisplayText>) -> Self {
        ConfirmationDialog {
            button,
            display_text,
        }
    }

    pub fn show(&self) {
        self.button.draw(); // Draw the button for the dialog
    }
}
