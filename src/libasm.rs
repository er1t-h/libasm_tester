use std::ops::Deref;

use libc::{c_char, c_int, c_void, size_t, ssize_t};
use libloading::Symbol;

use crate::LIBRARY;

macro_rules! function_wrapper {
    ($function_name: ident ($($param_name: tt: $param_type: ty),+ $(,)?) $(-> $return_type: ty)?) => {
        pub unsafe fn $function_name($($param_name: $param_type),+) $(-> $return_type)? {
            let name = stringify!($function_name);
            let f: Symbol<unsafe fn($($param_name: $param_type),+) $(-> $return_type)?> = LIBRARY.deref().get(name.as_bytes()).expect("function doesn't exist");
            f($($param_name),+)
        }
    };
}

#[repr(C)]
pub struct TList {
    pub data: *mut c_void,
    pub next: *mut TList,
}

function_wrapper!(ft_strlen(buffer: *const c_char) -> size_t);
function_wrapper!(ft_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char);
function_wrapper!(ft_strcmp(str1: *const c_char, str2: *const c_char) -> c_int);
function_wrapper!(ft_write(fd: c_int, buffer: *const c_void, length: usize) -> ssize_t);
function_wrapper!(ft_read(fd: c_int, buffer: *mut c_void, length: usize) -> ssize_t);
function_wrapper!(ft_strdup(s: *const c_char) -> *mut c_char);

function_wrapper!(ft_atoi_base(s: *const c_char, base: *const c_char) -> c_int);
function_wrapper!(ft_list_push_front(ls: *mut *mut TList, data: *mut c_void));
function_wrapper!(ft_list_size(ls: *const TList) -> size_t);
function_wrapper!(
    ft_list_remove_if(
        begin_list: *mut *mut TList,
        data_ref: *mut c_void,
        cmp: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
        free_fct: Option<unsafe extern "C" fn(*mut c_void)>
    )
);
function_wrapper!(
    ft_list_sort(
        begin_list: *mut *mut TList,
        cmp: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>
    )
);
