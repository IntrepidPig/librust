use crate::{fs::OpenError::PermissionDenied, prelude::*};

pub enum OpenError {
	DoesNotExist,
	PermissionDenied,
	Other(i32),
	Unknown,
}

impl OpenError {
	fn from_errno(e: i32) -> Self {
		match e {
			2 => OpenError::DoesNotExist,      // ENOENT
			13 => OpenError::PermissionDenied, // EACCES
			t => OpenError::Other(t),
		}
	}
}

pub unsafe fn open<P: FileDescriptorPermissions>(
	path: &[u8],
) -> Result<FileDescriptor<P>, OpenError> {
	match syscall(
		nr::OPEN,
		&[path.as_ptr() as usize, P::as_flags() as usize, 0666],
	) {
		Ok(fd) => Ok(FileDescriptor::new(fd as i32)),
		Err(e) => Err(OpenError::from_errno(e as i32)),
	}
}
