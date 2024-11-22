use crate::libasm;
use pretty_assertions::assert_eq;
use std::ffi::CString;

macro_rules! test {
    ($name: ident, $str: expr, $base: expr, $result: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let str = CString::new($str).expect("DPS: Cannot create str");
                let base = CString::new($base).expect("DPS: Cannot create base");
                let ret_val = unsafe { libasm::ft_atoi_base(str.as_ptr(), base.as_ptr()) };
                assert_eq!(ret_val, $result);
            }
        }
    };
}

test!(
    basic,
    "2147483647other_useless_thing",
    "0123456789",
    2147483647
);
test!(binary, "oooxxxxoxxoooxox_yup", "ox", 7877);
test!(
    b64,
    "    lasm+",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789:/",
    9808678
);
test!(
    plus_in_base,
    "Super",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    0
);
test!(
    neg_in_base,
    "Super",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-/",
    0
);
test!(
    double_in_base,
    "Super",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZAbcdefghijklmnopqrstuvwxyz0123456789:/",
    0
);
test!(nothing_in_base, "Super", "", 0);
test!(nothing_in_str, "", "0123456789", 0);

// How to add tests:
// `test!(name_of_the_test, "string to atoi", "the base", expected_result)`
