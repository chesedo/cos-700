use crate::abstract_factory_r;

trait Factory<T> {}
struct Button {}
struct Window {}

trait Gui
where
    Self: Factory<Button>,
    Self: Factory<Window>,
{
}

struct KDE {}

impl Gui for KDE {}

impl Factory<Button> for KDE {}
impl Factory<Window> for KDE {}
