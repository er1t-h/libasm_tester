use crate::{ft_list_push_front, ft_list_size, s_list};
use std::ffi::CString;

macro_rules! test {
    ($name: ident, $arg: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let mut list: *mut s_list = std::ptr::null::<s_list>() as *mut s_list;
                let all_data: Vec<CString> =
                    $arg.iter().map(|elt| CString::new(*elt).unwrap()).collect();
                for i in all_data.iter() {
                    unsafe {
                        ft_list_push_front(&mut list, i.as_ptr() as *mut cty::c_void);
                    }
                }
                let size = unsafe { ft_list_size(list) };
                assert_eq!(size, $arg.len());
                while (!list.is_null()) {
                    let tmp: *mut s_list = unsafe { std::ptr::read(&(list as *mut s_list)) };
                    list = unsafe { &mut *(*list).next };
                    unsafe {
                        libc::free(tmp as *mut cty::c_void);
                    }
                }
            }
        }
    };
}

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
