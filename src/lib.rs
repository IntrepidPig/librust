#![no_std]

use crate::prelude::*;

pub mod fs;
pub mod io;
pub mod fmt;
pub mod mem;
pub(crate) mod prelude {
	pub use crate::{fs::*, io::*, syscall::*, fmt::*, mem::*, *};
}

mod syscall;

pub fn exit(status: i32) -> ! {
	unsafe {
		syscall(nr::EXIT, &[status as usize]);
		core::hint::unreachable_unchecked();
	}
}