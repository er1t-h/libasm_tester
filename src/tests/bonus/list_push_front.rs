use crate::libasm::{self, TList};
use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

macro_rules! test {
    ($name: ident, $arg: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let arg = ["a"];
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

                let right_data: Vec<_> = arg
                    .iter()
                    .map(|x| String::from_utf8_lossy(x.as_bytes()).into_owned())
                    .rev()
                    .collect();
                let mut user_data = Vec::new();
                for _ in right_data.iter() {
                    let tmp: *mut TList = unsafe { std::ptr::read(&list) };
                    list = unsafe { &mut *(*list).next };
                    let content = unsafe { CStr::from_ptr((*tmp).data.cast()) };
                    user_data.push(String::from_utf8_lossy(content.to_bytes()).into_owned());
                    unsafe {
                        libc::free(tmp as *mut cty::c_void);
                    }
                }
                assert_eq!(
                    right_data, user_data,
                    "either you deleted too much elements, either not enough, either the wrong ones"
                );
            }
        }
    };
}

crate::fork_test! {
    #[test]
    fn with_list_as_null() {
        unsafe {
            libasm::ft_list_push_front(std::ptr::null_mut(), NonNull::dangling().as_mut());
        }
    }
}

test!(with_three_items, ["Yes", "Nope", "Meh"]);
test!(
    with_eight_items,
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
test!(with_one_item, ["Yes"]);

// How to add tests:
// `test!(name_of_the_test, ["str1", "str2", "str3", ...])`
