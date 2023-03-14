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
