mod strlen {
    use std::ffi::CString;
    use rusty_fork::rusty_fork_test;

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

	rusty_fork_test! {
		#[test]
		#[should_panic="signal: 11"]
		fn null() {
			let result = unsafe {
				ft_strlen(std::ptr::null())
			};
			assert_eq!(result, 0);
		}
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
					ft_strcpy(dest_ptr, src.as_ptr() as *mut i8)
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
					ft_strcmp(s1.as_ptr(), s2.as_ptr())
				};
				assert_eq!(ret_val, 0);
			}
		};
		($name: ident, positive, $str1: expr, $str2: expr) => {
			#[test]
			fn $name() {
				let s1 = CString::new($str1).expect("Cannot create first string");
				let s2 = CString::new($str2).expect("Cannot create second string");
				let ret_val = unsafe {
					ft_strcmp(s1.as_ptr(), s2.as_ptr())
				};
				assert!(ret_val < 0);
			}
		};
		($name: ident, negative, $str1: expr, $str2: expr) => {
			#[test]
			fn $name() {
				let s1 = CString::new($str1).expect("Cannot create first string");
				let s2 = CString::new($str2).expect("Cannot create second string");
				let ret_val = unsafe {
					ft_strcmp(s1.as_ptr(), s2.as_ptr())
				};
				assert!(ret_val > 0);
			}
		};
	}

	// Matching
	test!(basic, "SuperTest");
	test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
	test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");

	// Mismatch
	test!(basic_negative, negative, "SuperTest", "SuperTeste");
	test!(basic_positive, positive, "SuperTeste", "SuperTest");
	test!(longer_negative, negative, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Lorme ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
	test!(longer_positive, positive, "Lorme ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
	test!(utf8_negative, negative, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀓麻雀🀄がしたい。このテストは本当に面白いなぁ。");
	test!(utf8_positive, positive, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀓麻雀🀄がしたい。このテストは本当に面白いなぁ。", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");
}

mod write {
	use std::fs::File;
    use std::ffi::CString;
	use std::os::unix::io::AsRawFd;
    use cty::c_void;

    use crate::ft_write;

	macro_rules! test {
		($name: ident, $file_name: expr, $to_test: expr) => {
			#[test]
			fn $name() {
				let path = concat!("test_files/write/", $file_name, ".txt");
				let file = File::create(path).expect("Couldn't create file");
				let fd = file.as_raw_fd();
				let buffer = CString::new($to_test).expect("Couldn't create string");
				let length = $to_test.len();
				let ret_val = unsafe {
					ft_write(fd, buffer.as_ptr() as *mut c_void, length)
				};
				assert_eq!(ret_val, length as isize, "Return values do not match");
				let file_content = std::fs::read_to_string(path).expect("Couldn't read file");
				assert_eq!(file_content, $to_test, "The content of the file does not match to the given string. ");
			}
		};
	}

	test!(oui, "oui", "C'est vraiment genial le rust\n");
	test!(basic, "basic", "SuperTest\n");
	test!(longer, "longer", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.\n");
	test!(utf8, "utf8", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。\n");
}

mod read {
    use cty::c_void;
	use std::fs::File;
	use std::os::unix::io::AsRawFd;

    use crate::ft_read;

	macro_rules! test {
		($name: ident, $file_name: expr) => {
			#[test]
			fn $name() {
				let file_content = std::fs::read_to_string(concat!("test_files/read/", $file_name,".txt")).expect("Cannot read file");
				let file = File::open(concat!("test_files/read/", $file_name, ".txt")).expect("Cannot open file");
				let fd = file.as_raw_fd();
				let mut buffer = vec![0_u8; file_content.len()];
				let length = file_content.len();
				let ret_val = unsafe {
					ft_read(fd, buffer.as_mut_ptr() as *mut c_void, length)
				};
				assert_eq!(ret_val, length as isize);
				assert_eq!(buffer, file_content.as_bytes());
			}
		};
	}


	test!(basic, "basic");
	test!(longer, "longer");
	test!(utf8, "utf8");
}
