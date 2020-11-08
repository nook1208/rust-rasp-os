use crate::{cpu, println};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(m) = info.message() {
        println!("\nKernel Panic: {}", m);
    } else {
        println!("\nKernel Panic");
    }

    cpu::wait_forever()
}
