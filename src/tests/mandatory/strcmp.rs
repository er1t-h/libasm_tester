use crate::assert_same_sign;
use crate::libasm;
use std::ffi::CString;

///
/// This test asserts that the return of libc::strcmp and libasm::ft_strcmp
/// have the same sign.
///
macro_rules! test {
    ($name: ident, $str: expr) => {
        test!($name, $str, $str);
    };
    ($name: ident, $str1: expr, $str2: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let s1 = CString::new($str1).expect("Cannot create first string");
                let s2 = CString::new($str2).expect("Cannot create second string");
                eprintln!("tested string 1: `{}`", $str1);
                eprintln!("tested string 2: `{}`", $str2);
                let ret_val = unsafe {
                    libasm::ft_strcmp(s1.as_ptr(), s2.as_ptr())
                };
                let libc_val = unsafe {
                    libc::strcmp(s1.as_ptr(), s2.as_ptr())
                };
                assert_same_sign!(ret_val, libc_val);
            }
        }
    };
}

// Matching
test!(basic, "SuperTest");
test!(longer, include_str!("../../../test_files/longer.txt"));
test!(utf8, include_str!("../../../test_files/utf8.txt"));

// Mismatch
test!(with_same_first_letter, "SuperTest!", "Soit, je comprends");
test!(without_any_common_point, "SuperTest!", "Bof pas trop");
test!(with_short_positive_input, "SuperTeste", "SuperTest");
test!(with_short_negative_input, "SuperTest", "SuperTeste");

const FAILED_LOREM: &str = "Lorme ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.";
test!(
    with_longer_input_positive,
    FAILED_LOREM,
    include_str!("../../../test_files/longer.txt")
);
test!(
    with_longer_input_negative,
    include_str!("../../../test_files/longer.txt"),
    FAILED_LOREM
);

const FAILED_UTF8: &str = "Salut! C'est un test de qualité contenant de supers UTF-8. 🀓麻雀🀄がしたい。このテストは本当に面白いなぁ。";
test!(
    with_utf8_input_positive,
    FAILED_UTF8,
    include_str!("../../../test_files/utf8.txt")
);
test!(
    with_utf8_input_negative,
    include_str!("../../../test_files/utf8.txt"),
    FAILED_UTF8
);

// How to add new tests:
// if the two strings are identical: `test!(name_of_the_test, "the string that will be tested")`
// else: `test!(name_of_the_test, "string one", "string two")`
