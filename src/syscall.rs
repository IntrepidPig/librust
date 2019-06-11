pub use syscall::platform::nr;
use syscall::platform::*;

pub unsafe fn syscall(n: usize, args: &[usize]) -> Result<usize, usize> {
	let res = match args {
		&[] => syscall0(n),
		&[a] => syscall1(n, a),
		&[a, b] => syscall2(n, a, b),
		&[a, b, c] => syscall3(n, a, b, c),
		&[a, b, c, d] => syscall4(n, a, b, c, d),
		&[a, b, c, d, e] => syscall5(n, a, b, c, d, e),
		&[a, b, c, d, e, f] => syscall6(n, a, b, c, d, e, f),
		_ => return Err(0),
	};

	if res > core::mem::transmute::<_, usize>(-4096i64) {
		Err(-core::mem::transmute::<_, isize>(res) as usize)
	} else {
		Ok(res)
	}
}
