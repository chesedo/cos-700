use macro_patterns_dec::abstract_factory;
trait InternalAbstract: InternalFactory<Window> + InternalFactory<Group> + Display {}
