use macro_patterns_dec::visitor;

macro_rules! children_walker {
    ($var_name:ident, $visitor_name:ident) => {
        $var_name.get_children().iter().for_each(|child| {
            match child {
                Child::Button(button) => $visitor_name.visit_button(button.deref()),
                Child::Input(input) => $visitor_name.visit_input(input.deref()),
            };
        });
    };
}

visitor!(
    (&Window[ helper_tmpl = children_walker ]),
    (&dyn Group[ helper_tmpl = children_walker ]),
);
