use macro_patterns_dec::visitor;

visitor!(
    (&dyn Input),
    (&dyn Button),
    (&dyn Scrollbar),
    (&dyn SelectBox),
    (&dyn RadioButton),
    (&dyn Checkbox),
    (Window),
    (Group),
);
