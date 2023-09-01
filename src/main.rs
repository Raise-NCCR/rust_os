#![no_std] // rust の標準ライブラリを使用しない
#![no_main] // Rust ランタイムを使用しないので、main関数は使わない
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

#[panic_handler] // パニック時に呼び出す
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // 勝手に関数名を装飾しないようにする
// 大体のリンカが_start() をエントリーポイントとしている
// Cの呼び出し既約を使用するよう指定している
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}