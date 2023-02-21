mod atoi_base {
	use std::ffi::CString;

	use crate::ft_atoi_base;

	macro_rules! test {
		($name: ident, $str: expr, $base: expr, $result: expr) => {
			#[test]
			fn $name() {
				let str = CString::new($str).expect("Cannot create str");
				let base = CString::new($base).expect("Cannot create base");
				let ret_val = unsafe { ft_atoi_base(str.as_ptr(), base.as_ptr()) };
				assert_eq!(ret_val, $result);
			}
		};
	}

	test!(basic, "2147483647other_useless_thing", "0123456789", 2147483647);
	test!(binary, "oooxxxxoxxoooxox_yup", "ox", 7877);
	test!(b64, "    lasm+", "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789:/", 9808678);
	test!(plus_in_base, "Super", "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", 0);
	test!(neg_in_base, "Super", "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-/", 0);
	test!(double_in_base, "Super", "ABCDEFGHIJKLMNOPQRSTUVWXYZAbcdefghijklmnopqrstuvwxyz0123456789:/", 0);
	test!(nothing_in_base, "Super", "", 0);
	test!(nothing_in_str, "", "0123456789", 0);
}

// mod list_push_front {
// 	use crate::{list::FtList, ft_list_push_front};
// 	use std::ffi::CString;

// 	macro_rules! inside_test {
// 		($list: ident, $current: expr, $($other_args: expr),+) => {
// 			let data = CString::new($current).unwrap();
// 			unsafe {
// 				ft_list_push_front(&mut $list, data.as_ptr() as *const cty::c_void);
// 			}
// 			let elt: &FtList = inside_test!($list, $($other_args),+);
// 			let elt: &FtList = unsafe {&*elt.next};
// 			assert_eq!(unsafe {std::slice::from_raw_parts(elt.data as *mut u8, 3)}, $current.as_bytes());
// 			elt
// 		};
// 		($list: ident, $current: expr) => {{
// 			let data = CString::new($current).unwrap();
// 			unsafe {
// 				ft_list_push_front(&mut $list, data.as_ptr() as *const cty::c_void);
// 			}
// 			let elt: &FtList = unsafe {&*$list};
// 			assert_eq!(unsafe {std::slice::from_raw_parts(elt.data as *mut u8, 3)}, $current.as_bytes());
// 			elt
// 		}};
// 	}

// 	macro_rules! test {
// 		($name: ident, $($arg: expr),+) => {
// 			#[test]
// 			fn $name() {
// 				let mut list: *const FtList = std::ptr::null();
// 				inside_test!(list, $($arg),+);
// 			}
// 		};
// 	}

// 	test!(basic, "Yes", "Nope", "Meh");

// 	// #[test]
// 	// fn basic() {
// 	// 	let mut list: *const FtList = std::ptr::null();
// 	// 	let data1 = CString::new("Yes").unwrap();
// 	// 	unsafe {
// 	// 		ft_list_push_front(&mut list, data1.as_ptr() as *const cty::c_void);
// 	// 	}
// 	// 	let data2 = CString::new("Nope").unwrap();
// 	// 	unsafe {
// 	// 		ft_list_push_front(&mut list, data2.as_ptr() as *const cty::c_void);
// 	// 	}
// 	// 	let data3 = CString::new("Meh").unwrap();
// 	// 	unsafe {
// 	// 		ft_list_push_front(&mut list, data3.as_ptr() as *const cty::c_void);
// 	// 	}

// 	// 	let elt1 = unsafe {&*list};
// 	// 	let elt2 = unsafe {&*elt1.next};
// 	// 	let elt3 = unsafe {&*elt2.next};
// 	// 	assert_eq!(unsafe {std::slice::from_raw_parts(elt1.data as *mut u8, 3)}, data3.as_bytes());
// 	// 	assert_eq!(unsafe {std::slice::from_raw_parts(elt2.data as *mut u8, 4)}, data2.as_bytes());
// 	// 	assert_eq!(unsafe {std::slice::from_raw_parts(elt3.data as *mut u8, 3)}, data1.as_bytes());
// 	// }
// }
