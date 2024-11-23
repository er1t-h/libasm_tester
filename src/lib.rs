#[cfg(test)]
mod tests;
#[allow(dead_code)]
mod utils;

static LIBRARY: LazyLock<Library> = LazyLock::new(|| unsafe {
    Library::new(format!(
        "{}/libasm.so",
        current_dir()
            .expect("DPS: couldn't find the current directory")
            .display()
    ))
    .expect("DPS: couldn't load the dynamic library")
});

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod libasm;

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

use std::{env::current_dir, sync::LazyLock};

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

use libloading::Library;
