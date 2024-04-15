
#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod allocator;
mod frame;
mod hal_impl;
mod logging;

pub fn main(hart_id: usize) {
    println!("run default @ {}", hart_id);
}