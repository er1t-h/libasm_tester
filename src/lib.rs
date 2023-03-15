#[cfg(test)]
mod tests;

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
mod libasm;

pub use libasm::*;

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

#[allow(unused_macros)]
#[cfg(feature = "verbose")]
macro_rules! verbose {
	($($args: expr),+) => {
		println!($($args),+);
	};
}
#[allow(unused_macros)]
#[cfg(not(feature = "verbose"))]
macro_rules! verbose {
    ($($args: expr),+) => {};
}

#[allow(unused_imports)]
pub(crate) use verbose;
