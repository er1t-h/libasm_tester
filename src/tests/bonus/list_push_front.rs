use crate::{FtList, ft_list_push_front};
use std::ffi::CString;

macro_rules! test {
	($name: ident, $arg: expr) => {
		#[test]
		fn $name() {
			let mut list: *const FtList = std::ptr::null();
			let all_data: Vec<CString> = $arg.iter().map(|elt| CString::new(*elt).unwrap()).collect();
			for i in all_data.iter() {
				unsafe {
					ft_list_push_front(&mut list, i.as_ptr() as *const cty::c_void);
				}
			}
			let mut elt: &FtList = unsafe {&*list};
			for (index, i) in all_data.iter().rev().enumerate() {
				let content = unsafe {std::slice::from_raw_parts(elt.data as *mut u8, i.as_bytes().len())};
				println!("Index: {:3} ; Content: {}", index, String::from_utf8_lossy(&content));
				assert_eq!(content, i.as_bytes());
				let tmp: *mut FtList = unsafe { std::ptr::read(&((elt as *const FtList) as *mut FtList)) };
				elt = unsafe {&*elt.next};
				unsafe { libc::free(tmp as *mut cty::c_void); }
			}
		}
	};
}

test!(basic, ["Yes", "Nope", "Meh"]);
test!(more_items, ["Yes", "Nope", "Meh", "I dunno", "Lorem Ipsum", "Riichi Ippatsu Tsumo", "Ron", "ye"]);
test!(one_item, ["Yes"]);
