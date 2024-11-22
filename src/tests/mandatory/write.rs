use super::pipe;
use crate::libasm;
use pretty_assertions::{assert_eq, assert_str_eq};
use std::fs::File;
use std::io::Read;
use std::os::fd::FromRawFd;

///
/// This test will use `ft_write` to write into a Unix pipe, then read it using
/// Rust's functions. It will check the return value of `ft_write`, and compare
/// the buffer read from the pipe with the original input
///
macro_rules! test {
    ($name: ident, $to_test: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                unsafe {
                    let [read, write] = pipe();
                    let mut read = File::from_raw_fd(read);
                    let _write = File::from_raw_fd(write);
                    let to_write = $to_test;
                    let return_value = libasm::ft_write(write, to_write.as_ptr().cast(), to_write.len());
                    assert_eq!(return_value, to_write.len() as isize, "the return value should be equal to the buffer's len");
                    std::mem::drop(_write);
                    let mut buffer = Vec::with_capacity(to_write.len());
                    read.read_to_end(&mut buffer).expect("DPS: couldn't read from the pipe");
                    assert_str_eq!(String::from_utf8_lossy(to_write), String::from_utf8_lossy(buffer.as_slice()), "the two buffers are different");
                }
            }
        }
    };
}

crate::fork_test! {
    #[test]
    fn invalid_fd() {
        unsafe {
            let return_value = libasm::ft_write(-1, b"test".as_ptr().cast(), 4);
            assert_eq!(return_value, -1, "if the system call fail, it should return -1");
            assert_eq!(libc::EBADF, *libc::__errno_location());
        }
    }
}

test!(with_empty_string, b"");
test!(with_short_input, b"SuperTest\n");
test!(
    with_longer_input,
    include_bytes!("../../../test_files/longer.txt")
);
test!(
    with_utf8_input,
    include_bytes!("../../../test_files/utf8.txt")
);

// How to add new tests:
// Simply write `test!(name_of_the_test, b"the string that will be written")`
// If you want to add test using non-ascii characters, use `"the string that will be written".as_bytes()` instead.
