use macro_patterns::abstract_factory;

struct Button {}
struct Window {}
trait Factory<T> {}

abstract_factory!(Gui, Factory, Button, Window);
