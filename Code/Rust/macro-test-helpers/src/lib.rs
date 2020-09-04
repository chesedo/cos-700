use std::{
    fmt::Display,
    io::Write,
    process::{Command, Stdio},
    str,
};

mod print;
mod vec;
mod vec_hand;

/// Formats a [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) using `rustfmt`
/// # Example
/// ```
/// use macro_test_helpers::reformat;
///
/// assert_eq!(
///     reformat(&String::from("use std::{str,fmt};")),
///     "use std::{fmt, str};\n"
/// )
/// ```
pub fn reformat(text: &dyn Display) -> String {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to create command");
    {
        let stdin = rustfmt
            .stdin
            .as_mut()
            .expect("Failed to create input stream");
        stdin
            .write_all(text.to_string().as_bytes())
            .expect("Failed to write to input stream");
    }
    let output = rustfmt
        .wait_with_output()
        .expect("Format command did not end");
    String::from_utf8(output.stdout).expect("Failed to convert output to string")
}

/// Calls `cargo expand` on a module and returns the output as a vector of lines
pub fn expand_cli(module: &str) -> Vec<String> {
    let output = Command::new("cargo")
        .arg("expand")
        .arg(module)
        .output()
        .expect("Failed to expand module");

    let lines = str::from_utf8(&output.stdout[..])
        .expect("Failed to convert output to string")
        .lines();

    lines
        .skip(1)
        .take_while(|l| *l != "}")
        .map(|l| l[4..].to_owned())
        .collect()
}

/// Expands a modules and check that it equals some expected string
#[macro_export]
macro_rules! expand_eq {
    ($module:tt, $expected:tt) => {
        assert_eq!(
            expand_cli($module),
            $expected.lines().collect::<Vec<&str>>()
        );
    };
}

/// Checks that an expanded module equals the expanse of some `[module]_hand`
#[macro_export]
macro_rules! expand {
    ($module:tt) => {
        assert_eq!(
            expand_cli($module),
            expand_cli(&format!("{}_hand", $module))
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn reformat_simple() {
        assert_eq!(
            reformat(&String::from("trait Test< SomeType > { }")),
            "trait Test<SomeType> {}\n"
        );
    }

    #[test]
    fn expand_eq_print() {
        expand_eq!(
            "print",
            r#"#[allow(dead_code)]
fn test() {
    ::std::io::_print(::core::fmt::Arguments::new_v1(
        &["Test"],
        &match () {
            () => [],
        },
    ));
}
"#
        );
    }

    #[test]
    fn expand_vec() {
        expand!("vec");
    }
}
