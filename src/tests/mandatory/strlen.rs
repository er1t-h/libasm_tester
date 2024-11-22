use crate::libasm;
use pretty_assertions::assert_eq;
use std::ffi::CString;

///
/// This test uses `ft_strlen` to find the len of the string, then compare it with
/// the len of the Rust string.
///
macro_rules! test {
    ($name: ident, $to_test: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let test_str = CString::new($to_test).expect("Couldn't create string");
                let result = unsafe { libasm::ft_strlen(test_str.as_ptr()) };
                eprintln!("the string tested: `{}`", $to_test);
                assert_eq!(result, test_str.as_bytes().len(), "bad string length");
            }
        }
    };
}

test!(with_empty_string, "");
test!(with_short_string, "SuperTest");
test!(
    with_long_string,
    include_str!("../../../test_files/longer.txt")
);
test!(
    with_utf8_characters,
    include_str!("../../../test_files/utf8.txt")
);

// How to add new tests:
// Simply write `test!(name_of_the_test, "the string that will be tested")`
