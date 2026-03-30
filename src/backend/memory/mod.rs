use crate::backend::memory::pmm::BITMAP_PAGE;
use crate::backend::memory::vmm::MemMapper;
use core::ffi::c_void;
use linked_list_allocator::LockedHeap;
use crate::backend::serial::LogLevel::Info;
use crate::log;

pub mod pmm;
pub mod vmm;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn memset_u32(ptr: *mut u32, value: u32, count: usize) {
    let slice = core::slice::from_raw_parts_mut(ptr, count);
    slice.fill(value);
}

pub unsafe fn init_heap() {
    let heap_start = 0x40000000;
    let heap_end = 0x40100000;
    let heap_size = heap_end - heap_start;

    for p in (heap_start..heap_end).step_by(4096) {
        if let Some(frame) = BITMAP_PAGE.lock().alloc_frame() {
            log!(Info, "Mapping heap page: {:x}", p);
            MemMapper::mem_map(p, frame, 0x3);
        }
    }

    unsafe {
        ALLOCATOR
            .lock()
            .init(heap_start as *mut u8, heap_size as usize);
    }
}
