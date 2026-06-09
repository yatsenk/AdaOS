use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    PhysAddr,
    VirtAddr, 
    structures::paging::{
        FrameAllocator, 
        Mapper, 
        OffsetPageTable, 
        Page, 
        PageTable, 
        PageTableFlags, 
        PhysFrame, 
        Size4KiB
    },
};

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    pub fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(
            |r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(
            |r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(
            |r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    // FIXME: recreating usable_frame allocator on every allocation is not optimal
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let l4_table = active_level_4_table(physical_memory_offset);
        OffsetPageTable::new(l4_table, physical_memory_offset)
    }
}

// Delete this later
pub fn create_mapping(
    page: Page, 
    mapper: &mut OffsetPageTable, 
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: this is not safe
        mapper.map_to(page, frame, flags, frame_allocator)
    };

    map_to_result.expect("map_to failed").flush();
} 

/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(
    physical_memory_offset: VirtAddr,
) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    
    let phys_frame = level_4_page_table.start_address();
    let virt_addr = physical_memory_offset + phys_frame.as_u64();
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

// [DEPRECATED] use Translate::translate_addr
pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_frame;

    for &index in &table_indexes {
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable  = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None, 
            Err(FrameError::HugeFrame) => panic!("Huge pages are not supported"),
        }
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}
