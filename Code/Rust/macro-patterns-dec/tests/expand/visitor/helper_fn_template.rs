use macro_patterns_dec::visitor;

macro_rules! children_walker {
    ($var_name:ident) => {
        $var_name.get_children().iter().for_each(|child| {
            match child {
                Child::Button(button) => visitor.visit_button(button.deref()),
                Child::Input(input) => visitor.visit_input(input.deref()),
            };
        });
    };
}

visitor!(
    (&Window[ helper_tmpl = children_walker ]),
    (&dyn Group[ helper_tmpl = children_walker ]),
);
