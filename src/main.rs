#![no_std] // rust の標準ライブラリを使用しない
#![no_main] // Rust ランタイムを使用しないので、main関数は使わない
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

#[panic_handler] // パニック時に呼び出す
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // 勝手に関数名を装飾しないようにする
// 大体のリンカが_start() をエントリーポイントとしている
// Cの呼び出し既約を使用するよう指定している
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();
    
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}