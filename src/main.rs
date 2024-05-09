#![no_std]
#![no_main]
#![feature(panic_info_message)]

use polyhal::{get_mem_areas, PageAlloc, TrapFrame, TrapType};

mod allocator;
mod frame;
mod lang_items;
mod logging;

/// interrupt_handler
#[polyhal::arch_interrupt]
fn interrupt_handler(ctx: &mut TrapFrame, trap_type: TrapType) {
    println!("trap_type @ {:x?} {:#x?}", trap_type, ctx);
}

#[polyhal::arch_entry]
pub fn main(hart_id: usize) {
    println!("run default @ {}", hart_id);
    crate::allocator::init_allocator();
    crate::logging::init(Some("trace"));
    println!("init logging");

    // Init polyhal with page alloc, This init will init every platform
    polyhal::init(&PageAllocImpl);

    // get hardware available memory regions
    get_mem_areas().into_iter().for_each(|(start, size)| {
        println!("init memory region {:#x} - {:#x}", start, start + size);
        crate::frame::add_frame_range(start, start + size);
    });
}

struct PageAllocImpl;

impl PageAlloc for PageAllocImpl {
    fn alloc(&self) -> polyhal::addr::PhysPage {
        crate::frame::frame_alloc()
    }

    fn dealloc(&self, ppn: polyhal::addr::PhysPage) {
        crate::frame::frame_dealloc(ppn)
    }
}
