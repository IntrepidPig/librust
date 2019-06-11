use crate::prelude::*;

pub trait FileDescriptorPermissions {
	fn as_flags() -> i32;
}

pub struct ReadOnly;

impl FileDescriptorPermissions for ReadOnly {
	fn as_flags() -> i32 {
		00
	}
}

pub struct WriteOnly;

impl FileDescriptorPermissions for WriteOnly {
	fn as_flags() -> i32 {
		01
	}
}

pub struct FileDescriptor<P: FileDescriptorPermissions> {
	raw_fd: i32,
	phantom: core::marker::PhantomData<P>,
}

impl<P: FileDescriptorPermissions> FileDescriptor<P> {
	pub(crate) unsafe fn new(raw_fd: i32) -> Self {
		FileDescriptor {
			raw_fd,
			phantom: core::marker::PhantomData,
		}
	}
}

impl<P: FileDescriptorPermissions> Drop for FileDescriptor<P> {
	fn drop(&mut self) {
		unsafe {
			close(FileDescriptor::<ReadOnly>::new(self.raw_fd));
		}
	}
}

pub enum WriteError {
	/// Only managed to write this many bytes
	Incomplete(usize),
	Other(i32),
	Unknown,
}

impl WriteError {
	fn from_errno(e: i32) -> Self {
		match e {
			t => WriteError::Other(t),
		}
	}
}

pub unsafe fn write(fd: &FileDescriptor<WriteOnly>, buf: &[u8]) -> Result<(), WriteError> {
	match syscall(
		nr::WRITE,
		&[fd.raw_fd as usize, buf.as_ptr() as usize, buf.len()],
	) {
		Ok(bytes_written) => {
			if bytes_written != buf.len() {
				Err(WriteError::Incomplete(bytes_written))
			} else {
				Ok(())
			}
		}
		Err(e) => Err(WriteError::from_errno(e as i32)),
	}
}

pub unsafe fn close<P: FileDescriptorPermissions>(fd: FileDescriptor<P>) -> Result<(), ()> {
	match syscall(nr::CLOSE, &[fd.raw_fd as usize]) {
		Ok(_) => Ok(()),
		Err(e) => Err(()), // TODO CloseError
	}
}

pub unsafe fn print(msg: &str) -> Result<(), WriteError> {
	match syscall(nr::WRITE, &[1 as usize, msg.as_ptr() as usize, msg.len()]) {
		Ok(bytes_written) => {
			if bytes_written != msg.as_bytes().len() {
				Err(WriteError::Incomplete(bytes_written))
			} else {
				Ok(())
			}
		}
		Err(e) => Err(WriteError::from_errno(e as i32)),
	}
}
