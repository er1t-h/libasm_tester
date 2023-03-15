use crate::{ft_list_push_front, s_list, verbose};
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
                        ft_list_push_front(
                            &mut list as *mut *mut s_list,
                            i.as_ptr() as *mut cty::c_void,
                        );
                    }
                }
                let mut elt: &s_list = unsafe { &*list };
                for (_index, i) in all_data.iter().rev().enumerate() {
                    let content = unsafe {
                        std::slice::from_raw_parts(elt.data as *mut u8, i.as_bytes().len())
                    };
                    verbose!(
                        "Index: {:3} ; Content: {}",
                        _index,
                        String::from_utf8_lossy(&content)
                    );
                    assert_eq!(content, i.as_bytes());
                    let tmp: *mut s_list =
                        unsafe { std::ptr::read(&((elt as *const s_list) as *mut s_list)) };
                    elt = unsafe { &*elt.next };
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
