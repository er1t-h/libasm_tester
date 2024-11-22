use super::pipe;
use crate::libasm;
use libc::ssize_t;
use pretty_assertions::{assert_eq, assert_str_eq};
use std::fs::File;
use std::io::Write;
use std::os::fd::FromRawFd;

///
/// This test preset will write `file_content` into a Unix pipe, then use `ft_read`
/// on it.
///
/// It will check the content of the buffer, and the return value.
///
macro_rules! test {
    ($name: ident, $file_content: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                unsafe {
                    let [read, write] = pipe() ;
                    let mut write = File::from_raw_fd(write);
                    let _read = File::from_raw_fd(read); // To handle fd closure
                    let to_write = $file_content;
                    write.write_all(to_write).expect("DPS: couldn't write to the pipe");
                    let mut buffer = vec![0_u8; to_write.len()];
                    let return_value = libasm::ft_read(read, buffer.as_mut_ptr().cast(), to_write.len());
                    assert_eq!(return_value, to_write.len() as ssize_t, "the return value should be equal to the buffer's len");
                    assert_str_eq!(String::from_utf8_lossy(buffer.as_slice()), String::from_utf8_lossy(to_write), "the two buffers are different");
                }
            }
        }
    };
}

crate::fork_test! {
    #[test]
    fn invalid_fd() {
        unsafe {
            let mut buffer = vec![0_u8; 10];
            let return_value = libasm::ft_read(-1, buffer.as_mut_ptr().cast(), 10);
            assert_eq!(return_value, -1, "if the system call fail, it should return -1");
            assert_eq!(libc::EBADF, *libc::__errno_location());
        }
    }
}

test!(basic, b"SuperTest");
test!(longer, include_bytes!("../../../test_files/longer.txt"));
test!(utf8, include_bytes!("../../../test_files/utf8.txt"));

// How to add tests:
// Then write `test!(name_of_the_test, b"The string to read")`
// You can also write the text in separate files, then use include_bytes!("path/to/the/file")
