#[test]
pub fn pass_helpers() {
    macrotest::expand("tests/expand/helpers/*.rs");
}

#[test]
pub fn pass_abstract_factory() {
    macrotest::expand("tests/expand/abstract_factory/*.rs");
}

#[test]
pub fn pass_visitor() {
    macrotest::expand("tests/expand/visitor/*.rs");
}
