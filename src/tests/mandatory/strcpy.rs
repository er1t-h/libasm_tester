use crate::ft_strcpy;
use std::ffi::CString;

macro_rules! test {
    ($name: ident, $to_test: expr) => {
        crate::fork_test! {
            #[test]
            fn $name() {
                let src = CString::new($to_test).expect("Couldn't create string");
                let mut dest = [0_u8; $to_test.len() + 1];
                let dest_ptr = dest.as_mut_ptr() as *mut i8;
                let ret_val = unsafe { ft_strcpy(dest_ptr, src.as_ptr() as *mut i8) };
                let final_dest =
                    unsafe { std::slice::from_raw_parts(dest_ptr as *mut u8, $to_test.len() + 1) };
                let final_ret_val =
                    unsafe { std::slice::from_raw_parts(ret_val as *mut u8, $to_test.len() + 1) };
                assert_eq!(final_ret_val, final_dest);
                assert_eq!(final_dest, src.as_bytes_with_nul());
            }
        }
    };
}

test!(basic, "superTest");
test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");
