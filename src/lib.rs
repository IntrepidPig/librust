#![no_std]

use crate::prelude::*;

pub mod fs;
pub mod io;
pub(crate) mod prelude {
	pub use crate::{fs::*, io::*, syscall::*, *};
}

mod syscall;

pub fn exit(status: i32) -> ! {
	unsafe {
		syscall(nr::EXIT, &[status as usize]);
		core::hint::unreachable_unchecked();
	}
}
