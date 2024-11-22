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

crate::fork_test! {
    #[test]
    fn input_as_null() {
        let base = c"0123456789";
        let ret = unsafe {
            libasm::ft_atoi_base(std::ptr::null(), base.as_ptr())
        };
        assert_eq!(ret, 0);
    }

    #[test]
    fn base_as_null() {
        let number = c"2147483647";
        let ret = unsafe {
            libasm::ft_atoi_base(number.as_ptr(), std::ptr::null())
        };
        assert_eq!(ret, 0);
    }
}

test!(
    int_max_with_decimal_base,
    "2147483647other_useless_thing",
    "0123456789",
    2147483647
);
test!(binary_with_o_and_x, "oooxxxxoxxoooxox_yup", "ox", 7877);
test!(
    using_base64_like_base,
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
    minus_in_base,
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
test!(space_in_base, "123", "01234 56789", 0);
test!(tab_in_base, "123", "01234\t56789", 0);
test!(carriage_return_in_base, "123", "01234\r56789", 0);
test!(newline_in_base, "123", "01234\n56789", 0);
test!(nothing_in_base, "Super", "", 0);
test!(nothing_in_str, "", "0123456789", 0);

// How to add tests:
// `test!(name_of_the_test, "string to atoi", "the base", expected_result)`
