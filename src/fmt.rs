pub fn u64_to_str_radix(buf: &mut [u8], radix: u8, num: u64) -> Result<&mut [u8], ()> {
	const DIGITS: [u8; 16] = [
		b'0',
		b'1',
		b'2',
		b'3',
		b'4',
		b'5',
		b'6',
		b'7',
		b'8',
		b'9',
		b'a',
		b'b',
		b'c',
		b'd',
		b'e',
		b'f',
	];
	
	let mut numdiv = num;
	let mut i: usize = 0;
	
	if num == 0 {
		if buf.is_empty() {
			return Err(());
		}
		
		let last_index = buf.len() - 1;
		buf[last_index] = b'0';
		return Ok(&mut buf[last_index..]);
	}
	
	loop {
		if i == buf.len() {
			return Err(());
		}
		
		buf[buf.len() - 1 - i] = DIGITS[(numdiv % radix as u64) as usize];
		numdiv = numdiv / radix as u64;
		i += 1;
		
		if numdiv == 0 {
			let start_pos = buf.len() - i;
			return Ok(&mut buf[start_pos..]);
		}
	}
}