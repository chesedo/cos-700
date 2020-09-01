use crate::abstract_factory_r;

trait Factory<T> {}
struct Button {}
struct Window {}

abstract_factory_r!(Gui, Factory, Button, Window);

struct KDE {}

impl Gui for KDE {}

impl Factory<Button> for KDE {}
impl Factory<Window> for KDE {}
