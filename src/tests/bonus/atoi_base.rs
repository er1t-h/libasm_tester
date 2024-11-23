use crate::libasm;
use pretty_assertions::assert_eq;
use std::ffi::CString;

macro_rules! test {
    ($name: ident, $str: expr, $base: expr, $result: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let s = $str;
                let b = $base;
                let str = CString::new(s).expect("DPS: Cannot create str");
                let base = CString::new(b).expect("DPS: Cannot create base");
                eprintln!("input string: {}", s);
                eprintln!("input base: {}", b);
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
test!(form_feed_in_base, "123", "0123456789\x0c", 0);
test!(newline_in_base, "123", "01234\n56789", 0);
test!(carriage_return_in_base, "123", "01234\r56789", 0);
test!(horizontal_tab_in_base, "123", "01234\t56789", 0);
test!(vertical_tab_in_base, "123", "0123456\x0b789", 0);
test!(empty_base, "Super", "", 0);
test!(base_of_size_one, "Super", "0", 0);
test!(empty_string, "", "0123456789", 0);
test!(nothing_to_parse, "abcd", "0123456789", 0);
test!(int_min, "-2147483648oui", "0123456789", -2147483648);
test!(negative_number_with_base_6, "-deadbeef", "abcdef", -1030709);

// How to add tests:
// `test!(name_of_the_test, "string to atoi", "the base", expected_result)`
