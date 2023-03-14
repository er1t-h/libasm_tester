#[repr(C)]
pub struct FtList {
    pub data: *mut cty::c_void,
    pub next: *mut FtList
}

impl Drop for FtList {
    fn drop(&mut self) {
        if !self.next.is_null() {
            drop(self.next);
            unsafe { libc::free(self as *mut FtList as *mut cty::c_void); }
        }
    }
}

extern "C" {
    // Mandatory
    pub fn ft_strlen(str: *const cty::c_char) -> cty::size_t;
    pub fn ft_strcpy(dest: *mut cty::c_char, src: *mut cty::c_char) -> *mut cty::c_char;
    pub fn ft_strcmp(s1: *const cty::c_char, s2: *const cty::c_char) -> cty::c_int;
    pub fn ft_write(fd: cty::c_int, buffer: *mut cty::c_void, count: cty::size_t) -> cty::ssize_t;
    pub fn ft_read(fd: cty::c_int, buffer: *mut cty::c_void, count: cty::size_t) -> cty::ssize_t;
    pub fn ft_strdup(str: *const cty::c_char) -> *mut cty::c_char;

    // Bonus
    pub fn ft_atoi_base(str: *const cty::c_char, base: *const cty::c_char) -> cty::c_int;
    pub fn ft_list_push_front(begin: *mut *const FtList, data: *const cty::c_void);
    pub fn ft_list_size(begin: *const FtList) -> cty::size_t;
}

#[cfg(test)]
mod tests;

#[allow(unused_macros)]
macro_rules! assert_same_sign {
    ($lhs: expr, $rhs: expr) => {
        assert!(
            ($lhs > 0 && $rhs > 0) || ($lhs == 0 && $rhs == 0) || ($lhs < 0 && $rhs < 0),
            "\nError in assert_same_sign:\n\t left: `{}`\n\tright: `{}`\n",
            $lhs,
            $rhs
        );
    };
}

#[allow(unused_imports)]
pub(crate) use assert_same_sign;


#[allow(unused_macros)]
#[cfg(feature = "fork")]
macro_rules! fork_test {
    (#![rusty_fork(timeout_ms = $timeout: expr)]
     $(
        $(#[$meta:meta])*
        fn $test_name:ident() $body:block
    )*) => {
        rusty_fork::rusty_fork_test!{
            #![rusty_fork(timeout_ms = $timeout)]
            $(
                $(#[$meta])*
                fn $test_name() $body
            )*
        }
    };
    ($(
        $(#[$meta:meta])*
        fn $test_name:ident() $body:block
    )*) => {
        rusty_fork::rusty_fork_test!{
            #![rusty_fork(timeout_ms = 30000)]
            $(
                $(#[$meta])*
                fn $test_name() $body
            )*
        }
    };
}
#[allow(unused_macros)]
#[cfg(not(feature = "fork"))]
macro_rules! fork_test {
    (#![rusty_fork(timeout_ms = $timeout: expr)]
     $(
        $(#[$meta:meta])*
        fn $test_name:ident() $body:block
    )*) => {
        $(
            $(#[$meta])*
            fn $test_name() {
                $body
            }
        )*
    };
    ($(
        $(#[$meta:meta])*
        fn $test_name:ident() $body:block
    )*) => { $(
            $(#[$meta])*
            fn $test_name() {
                $body
            }
        )*
    };
}

#[allow(unused_imports)]
pub(crate) use fork_test;
