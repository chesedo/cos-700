use macro_patterns_dec::abstract_factory;
pub trait Abstract
where
    Self: Factory<Window>,
{
}
