use crate::libasm::{self, TList};
use std::ffi::CString;

macro_rules! test {
    ($name: ident, $arg: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let mut list: *mut TList = std::ptr::null::<TList>() as *mut TList;
                let all_data: Vec<CString> =
                    $arg.iter().map(|elt| CString::new(*elt).expect("DPS: Couldn't create a CString")).collect();
                for i in all_data.iter() {
                    unsafe {
                        libasm::ft_list_push_front(&mut list, i.as_ptr() as *mut cty::c_void);
                    }
                }
                let size = unsafe { libasm::ft_list_size(list) };
                assert_eq!(size, $arg.len(), "wrong list size");
                while (!list.is_null()) {
                    let tmp: *mut TList = unsafe { std::ptr::read(&(list as *mut TList)) };
                    list = unsafe { &mut *(*list).next };
                    unsafe {
                        libc::free(tmp as *mut cty::c_void);
                    }
                }
            }
        }
    };
}

test!(with_empty_list, [] as [&str; 0]);
test!(basic, ["Yes", "Nope", "Meh"]);
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
    ]
);
test!(one_item, ["Yes"]);
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
    ]
);

// How to add tests:
// `test!(name_of_the_test, ["str1", "str2", "str3", ...])`
