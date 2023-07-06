#![no_std] // rust の標準ライブラリを使用しない
#![no_main] // Rust ランタイムを使用しないので、main関数は使わない

use core::panic::PanicInfo;

#[panic_handler] // パニック時に呼び出す
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // 勝手に関数名を装飾しないようにする
// 大体のリンカが_start() をエントリーポイントとしている
// Cの呼び出し既約を使用するよう指定している
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}