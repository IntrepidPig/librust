use crate::prelude::*;
use core::{ptr::{self, NonNull},
mem::{self}};

static mut BRK: *const () = ptr::null();

#[repr(C)]
pub struct RegionHeader {
	pub size: usize,
	pub active: bool,
	pub prev: *mut RegionHeader,
	pub next: *mut RegionHeader,
}

const REGION_HEADER_SIZE: usize = mem::size_of::<RegionHeader>();

const MAX_ALIGN: usize = 16;

/// The first header of a region in the heap
static mut HEAP_START_HEADER: *mut RegionHeader = ptr::null_mut();
/// The last header of a region in the heap
static mut HEAP_END_HEADER: *mut RegionHeader = ptr::null_mut();

/// Returns a pointer to a new space on the heap of `size` bytes, preceded by a RegionHeader.
/// Properly updates all pointers in the tracked region list.
pub unsafe fn allocate(size: usize) -> Result<*mut (), ()> {
	let full_size = size + REGION_HEADER_SIZE;
	
	let region = if let Some(region) = find_inactive_region_of_size(size) {
		(*region).active = true;
		// split this region into an active one and an inactive one if there's space
		if (*region).size > size + REGION_HEADER_SIZE {
			let old_size = (*region).size;
			(*region).size = size;
			let new_region = (region as usize + full_size) as *mut RegionHeader;
			ptr::write(new_region, RegionHeader {
				size: old_size - size - REGION_HEADER_SIZE,
				active: false,
				prev: region,
				next: (*region).next,
			});
			(*region).next = new_region;
			if !(*new_region).next.is_null() {
				(*(*new_region).next).prev = new_region;
			}
			
			if region == HEAP_END_HEADER {
				HEAP_END_HEADER = new_region;
			}
		}
		region
	} else {
		let new_ptr = sbrk(full_size)? as *mut RegionHeader;
		ptr::write(new_ptr, RegionHeader {
			size,
			active: true,
			prev: HEAP_END_HEADER,
			next: ptr::null_mut(),
		});
		if !HEAP_END_HEADER.is_null() {
			(*HEAP_END_HEADER).next = new_ptr;
			HEAP_END_HEADER = new_ptr;
		} else {
			HEAP_END_HEADER = new_ptr;
			HEAP_START_HEADER = new_ptr;
		}
		new_ptr
	};
	
	Ok(((region as usize) + REGION_HEADER_SIZE) as *mut _)
}

/// Marks the region of the pointer as inactive
pub unsafe fn deallocate<T: ?Sized>(ptr: *mut T) -> Result<(), ()> {
	// Locate the header of the region and mark it as inactive
	let original_region = ((ptr as *mut () as usize) - REGION_HEADER_SIZE) as *mut RegionHeader;
	(*original_region).active = false;
	
	// Check if it has adjacent free blocks and combine them if so
	// Check the previous block
	let preceding_region = (*original_region).prev;
	let mut consumed_prev = false;
	if !preceding_region.is_null() {
		if !(*preceding_region).active {
			(*preceding_region).size += REGION_HEADER_SIZE + (*original_region).size;
			(*preceding_region).next = (*original_region).next;
			(*(*preceding_region).next).prev = preceding_region;
			consumed_prev = true;
		}
	}
	
	// Check the next block
	// new_region is is the location of the new region, depending on whether the original one
	// was combined with its preceding region or not.
	let new_region = if consumed_prev { preceding_region } else { original_region };
	let following_region = (*new_region).next;
	if !following_region.is_null() {
		if !(*following_region).active {
			(*new_region).size += REGION_HEADER_SIZE + (*following_region).size;
			(*new_region).next = (*following_region).next;
			if !(*new_region).next.is_null() {
				(*(*new_region).next).prev = new_region;
			}
		}
	}
	Ok(())
}

fn find_inactive_region_of_size(size: usize) -> Option<*mut RegionHeader> {
	unsafe {
		let mut found_region: Option<*mut RegionHeader> = None;
		for_each_region(|region| {
			if (*region).size >= size && !(*region).active {
				found_region = Some(region);
				false
			} else {
				true
			}
		});
		found_region
	}
}

/// `f` should return true to continue looping or false to stop after the current element
pub unsafe fn for_each_region<F: FnMut(*mut RegionHeader) -> bool>(mut f: F) {
	unsafe {
		let mut region_ptr = HEAP_START_HEADER;
		loop {
			if region_ptr.is_null() {
				break;
			}
			
			if !f(region_ptr) {
				break;
			};
			region_ptr = (*region_ptr).next;
		}
	}
}

unsafe fn sbrk(increment: usize) -> Result<*mut (), ()> {
	let start = brk_sys(ptr::null()).unwrap_err();
	match brk_sys((start as usize + increment) as *const ()) {
		Ok(_) => Ok(start),
		Err(_) => Err(()),
	}
}

unsafe fn brk(addr: *const ()) -> Result<(), ()> {
	brk_sys(addr).map(|_| ()).map_err(|_| ())
}

/// Direct clone of the kernel brk syscall. On success, it returns the address of the new break.
/// On failure, it returns the value of the old break. The current break can be obtained by passing
/// a guaranteed invalid argument such as null. (The argument is the intended new location of the
/// break.)
unsafe fn brk_sys(addr: *const ()) -> Result<*mut (), *mut ()> {
	let res = syscall_raw(nr::BRK, &[addr as usize]) as *mut ();
	if res == addr as *mut _ {
		Ok(res)
	} else {
		Err(res)
	}
}
