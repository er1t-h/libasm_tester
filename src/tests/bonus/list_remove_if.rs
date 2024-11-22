use crate::{
    libasm::{self, TList},
    utils,
};
use std::ffi::{CStr, CString};

macro_rules! test {
    ($name: ident, $arg: expr, $to_remove: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let arg = $arg;
                let mut list: *mut TList = std::ptr::null::<TList>() as *mut TList;
                let all_data: Vec<CString> = arg
                    .iter()
                    .map(|elt| CString::new(*elt).expect("DPS: couldn't create a CString"))
                    .collect();
                for i in all_data.iter() {
                    unsafe {
                        libasm::ft_list_push_front(&mut list, i.as_ptr() as *mut cty::c_void);
                    }
                }
                let tmp = CString::new($to_remove).expect("DPS: couldn't create a CString");
                unsafe {
                    libasm::ft_list_remove_if(
                        &mut list,
                        tmp.as_ptr() as *mut libc::c_void,
                        utils::compare_first_letter,
                        utils::no_free,
                    )
                }

                let right_data: Vec<_> = arg
                    .iter()
                    .filter(|elt| elt.chars().nth(0) != $to_remove.chars().nth(0))
                    .map(|x| String::from_utf8_lossy(x.as_bytes()).into_owned())
                    .rev()
                    .collect();
                let mut user_data = Vec::new();
                for _ in right_data.iter()
                {
                    let tmp: *mut TList = unsafe { std::ptr::read(&(list as *mut TList)) };
                    list = unsafe { &mut *(*list).next };
                    let content = unsafe {
                        CStr::from_ptr((*tmp).data.cast())
                    };
                    user_data.push(String::from_utf8_lossy(content.to_bytes()).into_owned());
                    unsafe {
                        libc::free(tmp as *mut cty::c_void);
                    }
                }
                assert_eq!(right_data, user_data, "either you deleted too much elements, either not enough, either the wrong ones");
            }
        }
    };
}

test!(basic, ["Yup", "Nope", "Yup"], "Nope");
test!(
    more_items,
    [
        "Yes",
        "Nope",
        "Meh",
        "I dunno",
        "Lorem Ipsum",
        "Riichi Ippatsu Tsumo",
        "Ron",
        "ye"
    ],
    "Rope"
);
test!(one_item, ["Yes"], "Yes");
test!(
    hundred,
    [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
        "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
        "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46",
        "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61",
        "62", "63", "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76",
        "77", "78", "79", "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91",
        "92", "93", "94", "95", "96", "97", "98", "99", "100"
    ],
    "11037"
);

// How to add tests:
// `test!(name_of_the_test, ["str1", "str2", "str3", ...], "data_reference")`
// Every items with the same first character as data_reference will be deleted
