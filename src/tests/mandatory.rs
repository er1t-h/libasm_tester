mod strlen {
    use std::ffi::CString;
    use crate::ft_strlen;

	macro_rules! test {
		($name: ident, $to_test: expr) => {
			#[test]
			fn $name() {
				let test_str = CString::new($to_test).expect("Couldn't create string");
				let result = unsafe {
					ft_strlen(test_str.as_ptr())
				};
				assert_eq!(result, test_str.as_bytes().len());
			}
		};
	}

	test!(basic, "SuperTest");
	test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
	test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");
}

mod strcpy {
    use std::ffi::CString;
    use crate::ft_strcpy;

	macro_rules! test {
		($name: ident, $to_test: expr) => {
			#[test]
			fn $name() {
				let src = CString::new($to_test).expect("Couldn't create string");
				let mut dest = [0_u8; $to_test.len() + 1];
				let dest_ptr = dest.as_mut_ptr() as *mut i8;
				let ret_val = unsafe {
					ft_strcpy(dest_ptr, src.clone().into_raw())
				};

				let final_dest = unsafe {std::slice::from_raw_parts(dest_ptr as *mut u8, $to_test.len() + 1)};
				let final_ret_val = unsafe {std::slice::from_raw_parts(ret_val as *mut u8, $to_test.len() + 1)};
				assert_eq!(final_ret_val, final_dest);
				assert_eq!(final_dest, src.as_bytes_with_nul());
			}
		};
	}

	test!(basic, "superTest");
	test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
	test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");
}

mod strcmp {
	use std::ffi::CString;
    use crate::ft_strcmp;

	macro_rules! test {
		($name: ident, $to_test: expr) => {
			#[test]
			fn $name() {
				let s1 = CString::new($to_test).expect("Cannot create first string");
				let s2 = CString::new($to_test).expect("Cannot create second string");
				let ret_val = unsafe {
					ft_strcmp(s1.into_raw(), s2.into_raw())
				};
				assert_eq!(ret_val, 0);
			}
		};
		($name: ident, $str1: expr, $str2: expr) => {
			#[test]
			fn $name() {
				let s1 = CString::new($str1).expect("Cannot create first string");
				let s2 = CString::new($str2).expect("Cannot create second string");
				let ret_val = unsafe {
					ft_strcmp(s1.into_raw(), s2.into_raw())
				};
				assert_ne!(ret_val, 0);
			}
		};
	}

	// Matching
	test!(basic, "SuperTest");
	test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
	test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");

	// Mismatch
	test!(basic_mismatch, "SuperTest", "SuperTeste");
	test!(longer_mismatch, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Lorme ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
	test!(utf8_mismatch, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀓麻雀🀄がしたい。このテストは本当に面白いなぁ。");
}

mod write {
    use std::ffi::CString;
    use cty::c_void;

    use crate::ft_write;

	macro_rules! test {
		($name: ident, $to_test: expr) => {
			#[test]
			fn $name() {
				let fd = 1;
				let buffer = CString::new($to_test).expect("Couldn't create string");
				let length = $to_test.len();
				let ret_val = unsafe {
					ft_write(fd, buffer.into_raw() as *mut c_void, length)
				};
				assert_eq!(ret_val, length as isize);
			}
		};
	}

	test!(basic, "SuperTest\n");
	test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.\n");
	test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。\n");
}

mod read {
    use std::ffi::CString;
    use cty::c_void;

    use crate::ft_read;

	macro_rules! test {
		($name: ident, $to_test: expr) => {
		};
	}

	#[test]
	fn basic() {
		let fd = 1;
		let mut buffer = [0_i8; 10];
		let length = 10;
		let ret_val = unsafe {
			ft_read(fd, buffer.as_mut_ptr() as *mut c_void, length)
		};
		assert_eq!(ret_val, length as isize);
	}

	test!(basic, "SuperTest\n");
	test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.\n");
	test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。\n");
}
