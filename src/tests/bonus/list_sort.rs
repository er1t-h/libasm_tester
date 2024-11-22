use std::ffi::{CStr, CString};

use crate::{
    libasm::{self, TList},
    utils,
};
use pretty_assertions::assert_eq;

macro_rules! test {
    ($name:ident, $value: expr) => {
        crate::fork_test! {
            #[test]
            fn $name(){
                let mut list: *mut TList = std::ptr::null::<TList>() as *mut TList;
                let mut all_data: Vec<CString> =
                    $value.iter().map(|elt| CString::new(*elt).expect("DPS: couldn't create a CString")).collect();
                for i in all_data.iter() {
                    unsafe {
                        libasm::ft_list_push_front(&mut list, i.as_ptr() as *mut libc::c_void);
                    }
                }
                unsafe { libasm::ft_list_sort(&mut list, utils::strcmp_wrapper) }

                all_data.sort();
                let mut user_sorted_data = Vec::new();
                for _ in all_data.iter()
                {
                    let tmp: *mut TList = unsafe { std::ptr::read(&(list as *mut TList)) };
                    list = unsafe { &mut *(*list).next };
                    let content = unsafe {
                        CStr::from_ptr((*tmp).data.cast())
                    };
                    user_sorted_data.push(String::from_utf8_lossy(content.to_bytes()).into_owned());
                    unsafe {
                        libc::free(tmp as *mut libc::c_void);
                    }
                }
                let all_string: Vec<_> = all_data.into_iter().map(|x| String::from_utf8_lossy(x.as_bytes()).into_owned()).collect();
                assert_eq!(
                    all_string,
                    user_sorted_data,
                    "the array is not sorted"
                );
            }
        }
    };
}

test!(
    basic,
    [
        "Bonjour",
        "je vais bien",
        "J'adore l'ASM",
        "Riichi",
        "Ippatsu"
    ]
);
test!(
    ten_items,
    ["4", "1", "5", "7", "0", "9", "3", "8", "2", "6"]
);
test!(
    hundred,
    [
        "9", "15", "26", "36", "37", "38", "23", "31", "21", "5", "2", "10", "11", "7", "6", "22",
        "12", "32", "33", "24", "3", "25", "39", "1", "4", "14", "16", "34", "30", "47", "50",
        "45", "61", "13", "46", "29", "48", "49", "72", "73", "19", "20", "28", "27", "8", "40",
        "42", "43", "58", "59", "78", "44", "79", "80", "41", "53", "54", "55", "85", "86", "87",
        "88", "35", "81", "82", "71", "51", "17", "18", "52", "60", "63", "64", "66", "89", "99",
        "100", "65", "76", "62", "83", "84", "77", "56", "57", "67", "68", "69", "70", "94", "95",
        "98", "96", "93", "74", "75", "97", "90", "91", "92",
    ]
);

// How to add tests:
// `test!(name_of_the_test, ["str1", "str2", "str3", ...])`
