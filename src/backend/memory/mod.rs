use crate::backend::memory::pmm::BITMAP_PAGE;
use crate::backend::memory::vmm::MemMapper;
use linked_list_allocator::LockedHeap;

pub mod pmm;
pub mod vmm;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn memset_u32(ptr: *mut u32, value: u32, count: usize) {
    let slice = core::slice::from_raw_parts_mut(ptr, count);
    slice.fill(value);
}

pub unsafe fn init_heap() {
    let heap_start = 0x10000000;
    let heap_size = 16 * 1024 * 1024; // 16 MB
    let heap_end = heap_start + heap_size;

    for p in (heap_start..heap_end).step_by(4096) {
        let frame = BITMAP_PAGE
            .lock()
            .alloc_frame()
            .expect("OUT OF PHYSICAL MEMORY: Increase QEMU RAM (-m 512M)");

        MemMapper::mem_map(p, frame, 0x3);
    }

    unsafe {
        ALLOCATOR
            .lock()
            .init(heap_start as *mut u8, heap_size as usize);
    }
}
