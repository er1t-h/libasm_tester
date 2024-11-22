//!
//! This module contains example of functions to pass as function pointers for
//! functions like `ft_list_remove_if`
//!

use libc::{c_char, c_int, c_void};

///
/// Compares only the first letter of the word
///
pub unsafe extern "C" fn compare_first_letter(s1: *const c_void, s2: *const c_void) -> c_int {
    let s1 = s1.cast::<c_char>();
    let s2 = s2.cast::<c_char>();
    (*s1 - *s2) as c_int
}

///
/// No-op function
///
pub unsafe extern "C" fn no_free(_mem: *mut c_void) {}

///
/// Calls `strcmp` (useful because it accepts *const c_void)
///
pub unsafe extern "C" fn strcmp_wrapper(s1: *const c_void, s2: *const c_void) -> c_int {
    libc::strcmp(s1.cast(), s2.cast())
}
