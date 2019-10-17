extern crate lcd;

use std::fmt;

macro_rules! pretty_assert_eq {
    ($left:expr, $right:expr) => {
        assert_eq!(PrettyString($left), PrettyString($right));
    };
}

const SIMPLE: &str = r#"
 _     _  _     _  _  _  _  _
| |  | _| _||_||_ |_   ||_||_|
|_|  ||_  _|  | _||_|  ||_| _|
"#;

#[test]
fn test_simple_size() {
    pretty_assert_eq!(
        SIMPLE.trim_start_matches('\n'),
        &lcd::run(123456789, 10, 1, 1)
    )
}

const DOUBLE: &str = r#"
 __      __  __      __  __  __  __  __
|  |   |   |   ||  ||   |      ||  ||  |
|  |   | __| __||__||__ |__    ||__||__|
|  |   ||      |   |   ||  |   ||  |   |
|__|   ||__  __|   | __||__|   ||__| __|
"#;

#[test]
fn test_double_size() {
    pretty_assert_eq!(
        DOUBLE.trim_start_matches('\n'),
        &lcd::run(123456789, 10, 2, 2)
    )
}

#[derive(PartialEq, Eq)]
pub struct PrettyString<'a>(pub &'a str);

impl<'a> fmt::Debug for PrettyString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("\n")?;
        f.write_str(self.0)
    }
}
