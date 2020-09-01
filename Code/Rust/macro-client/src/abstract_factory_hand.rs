use macro_patterns::abstract_factory;

struct Button {}
struct Window {}
trait Factory<T> {}

trait Gui: Factory<Button> + Factory<Window> {}
