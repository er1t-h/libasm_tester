use cty::c_void;
use std::ffi::CString;
use std::fs::File;
use std::os::unix::io::AsRawFd;

use crate::ft_write;

macro_rules! test {
    ($name: ident, $file_name: expr, $to_test: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let path = concat!("test_files/write/", $file_name, ".txt");
                let file = File::create(path).expect("Couldn't create file");
                let fd = file.as_raw_fd();
                let buffer = CString::new($to_test).expect("Couldn't create string");
                let length = $to_test.len();
                let ret_val = unsafe { ft_write(fd, buffer.as_ptr() as *mut c_void, length) };
                assert_eq!(ret_val, length as isize, "Return values do not match");
                let file_content = std::fs::read_to_string(path).expect("Couldn't read file");
                assert_eq!(
                    file_content, $to_test,
                    "The content of the file does not match to the given string. "
                );
            }
        }
    };
}

test!(oui, "oui", "C'est vraiment genial le rust\n");
test!(basic, "basic", "SuperTest\n");
test!(longer, "longer", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.\n");
test!(utf8, "utf8", "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。\n");
