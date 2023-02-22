#[cfg(test)]
mod tests;

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
mod libasm;

pub use libasm::*;

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
