#![no_std]
#![no_main]
mod writer;
use writer::FrameBufferWriter;

//didnt need the below import again due to the use of the print macro
//use core::fmt::Write;

use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;
//Use the entry_point macro to register the entry point function:
// bootloader_api::entry_point!(kernel_main)
//optionally pass a custom config

macro_rules! print {
    ($writer:expr, $fmt:expr $(, $args:expr)* ) => {{
        use core::fmt::Write;
        write!($writer, $fmt $(, $args)*).unwrap();
    }};
}

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
bootloader_api::entry_point!(my_entry_point, config =
&BOOTLOADER_CONFIG);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}


fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut(); let mut frame_buffer_writer =
    FrameBufferWriter::new(buffer, frame_buffer_info);//below requires this
    frame_buffer_writer.set_cursor_position(0, 0); //function to allow user manually set cursor position
    print!(frame_buffer_writer, "\\cColor \\cApplied \n");
    print!(frame_buffer_writer, "\n");
    print!(frame_buffer_writer, "This is the code...\nIt works with escape keys...\tThis is a tab\n");

    //testing whether it wraps on the line
    print!(frame_buffer_writer, "1gddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd");
    loop {
        hlt();//stop x86_64 from being unnecessarily busy while looping
    }
}
