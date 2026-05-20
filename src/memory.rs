use x86_64::{
    VirtAddr, 
    structures::paging::PageTable,
};

pub unsafe fn active_level_4_table(
    physical_memory_offset: VirtAddr,
) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    
    let phys_frame = level_4_page_table.start_address();
    let virt_addr = physical_memory_offset + phys_frame.as_u64();
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}
