use crate::libasm;
use pretty_assertions::{assert_eq, assert_str_eq};
use std::ffi::CString;

///
/// This tests will `ft_strcpy` the expected string to an empty buffer.
/// It will then assert that the returned pointer is equal to the given `dest` pointer
/// And it will compare the two buffers
///
macro_rules! test {
    ($name: ident, $to_test: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let src = CString::new($to_test).expect("DPS: Couldn't create string");
                let mut dest = [55_u8; $to_test.len() + 1]; // putting 55 to ensure that ft_strcpy copies the \0 character
                let dest_ptr = dest.as_mut_ptr() as *mut i8;
                let ret_val = unsafe { libasm::ft_strcpy(dest_ptr, src.as_ptr() as *mut i8) };
                assert_eq!(dest_ptr, ret_val, "the destination pointer and the returned pointer are different");
                assert_str_eq!($to_test, String::from_utf8_lossy(&dest[..dest.len() - 1]), "the buffer wasn't copied as expected");
                assert!(dest.last() == Some(&0), r"the \0 character was not copied")
            }
        }
    };
}

test!(with_empty_string, "");
test!(with_short_input, "superTest");
test!(
    with_longer_input,
    include_str!("../../../test_files/longer.txt")
);
test!(
    with_utf8_input,
    include_str!("../../../test_files/utf8.txt")
);

// How to add new tests:
// Simply write `test!(name_of_the_test, "the string that will be tested")`
