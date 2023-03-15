use std::ffi::CString;

use crate::{ft_list_push_front, ft_list_sort, s_list, strcmp_wrapper};

macro_rules! test {
    ($name:ident, $value: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let mut list: *mut s_list = std::ptr::null::<s_list>() as *mut s_list;
                let mut all_data: Vec<CString> =
                    $value.iter().map(|elt| CString::new(*elt).unwrap()).collect();
                for i in all_data.iter() {
                    unsafe {
                        ft_list_push_front(&mut list, i.as_ptr() as *mut cty::c_void);
                    }
                }
                unsafe { ft_list_sort(&mut list, Some(strcmp_wrapper)) }

                all_data.sort();
                for i in all_data
                    .iter()
                {
                    let tmp: *mut s_list = unsafe { std::ptr::read(&(list as *mut s_list)) };
                    list = unsafe { &mut *(*list).next };
                    let content = unsafe {
                        std::slice::from_raw_parts((*tmp).data as *mut u8, i.as_bytes().len())
                    };
                    crate::verbose!("Content: {:?}", content);
                    assert_eq!(content, i.as_bytes());
                    unsafe {
                        libc::free(tmp as *mut cty::c_void);
                    }
                }
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
