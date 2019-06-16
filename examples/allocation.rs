use core::{
	mem,
	ptr::{self, NonNull},
};

use librust::{
	mem::{allocate, deallocate, for_each_region},
	io,
	fmt::{u64_to_str_radix},
};

fn main() {
	unsafe {
		const ARR_SIZE: usize = 1024 * 1024 * 1024 * 2;
		let mut big_arr: Option<NonNull<[u8; ARR_SIZE]>> = None; // 2GiB
		match allocate(mem::size_of::<[u8; ARR_SIZE]>()) {
			Ok(ptr) => {
				big_arr = NonNull::new(ptr as *mut _);
			},
			Err(_) => {
				panic!("Failed to allocate")
			},
		}
		let big_arr: NonNull<[u8; ARR_SIZE]> = big_arr.unwrap();
		let ptr: *mut [u8; ARR_SIZE] = big_arr.as_ptr();
		*(ptr as *mut u8) = 1u8;
		*((ptr as usize + ARR_SIZE - 1) as *mut u8) = 255u8;
		
		print_regions();
		io::print("\n");
		let to_free = allocate(200).unwrap();
		print_regions();
		io::print("\n");
		deallocate(to_free);
		print_regions();
		io::print("\n");
		allocate(25);
		print_regions();
		io::print("\n");
		allocate(50);
		print_regions();
		io::print("\n");
		allocate(512);
		print_regions();
		io::print("\n");
		let first = allocate(1024).unwrap();
		let second = allocate(1024).unwrap();
		let third = allocate(1024).unwrap();
		print_regions();
		io::print("\n");
		deallocate(first);
		print_regions();
		io::print("\n");
		deallocate(third);
		print_regions();
		io::print("\n");
		deallocate(second);
		print_regions();
		io::print("\n");
		
		loop {}
	}
}

unsafe fn print_regions() {
	let mut i = 0;
	let mut buf = [0u8; 128];
	for_each_region(|region| {
		i += 1;
		io::print("Region Header ");
		let minibuf = u64_to_str_radix(&mut buf, 10, i).unwrap();
		io::print(core::str::from_utf8_unchecked(minibuf));
		io::print(":\n\tLocation: 0x");
		let minibuf = u64_to_str_radix(&mut buf, 16, region as u64).unwrap();
		io::print(core::str::from_utf8_unchecked(minibuf));
		io::print("\n\tSize: ");
		let minibuf = u64_to_str_radix(&mut buf, 10, (*region).size as u64).unwrap();
		io::print(core::str::from_utf8_unchecked(minibuf));
		io::print("\n\tActive: ");
		if (*region).active {
			io::print("true\n");
		} else {
			io::print("false\n");
		}
		
		true
	});
}