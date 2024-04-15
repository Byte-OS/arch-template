use core::panic::PanicInfo;

use arch::addr::PhysPage;
use arch::{api::ArchInterface, TrapFrame, TrapType};
use arch::shutdown;
use crate_interface::impl_interface;
use fdt::node::FdtNode;
use log::info;

use crate::println;

pub struct ArchInterfaceImpl;

#[impl_interface]
impl ArchInterface for ArchInterfaceImpl {
    /// Init allocator
    fn init_allocator() {
        crate::allocator::init_allocator();
    }
    /// kernel interrupt
    fn kernel_interrupt(ctx: &mut TrapFrame, trap_type: TrapType) {
        println!("trap_type @ {:x?} {:#x?}", trap_type, ctx);
    }
    /// init log
    fn init_logging() {
        crate::logging::init(Some("trace"));
        println!("init logging");
    }
    /// add a memory region
    fn add_memory_region(start: usize, end: usize) {
        println!("init memory region {:#x} - {:#x}", start, end);
        crate::frame::add_frame_range(start, end);
    }
    /// kernel main function, entry point.
    fn main(hartid: usize) {
        crate::main(hartid);
        
        info!("shutdown!!!!");
        shutdown();
    }
    /// Alloc a persistent memory page.
    fn frame_alloc_persist() -> PhysPage {
        crate::frame::frame_alloc()
    }
    /// Unalloc a persistent memory page
    fn frame_unalloc(ppn: PhysPage) {
        crate::frame::frame_dealloc(ppn)
    }
    /// Preprare drivers.
    fn prepare_drivers() {
        println!("prepare drivers");
    }
    /// Try to add device through FdtNode
    fn try_to_add_device(_fdt_node: &FdtNode) {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log::error!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        log::error!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
