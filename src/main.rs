#![no_std] // Não vincule a biblioteca padrão do Rust
#![no_main] // desativar todos os pontos de entrada no nível Rust

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // não altere (mangle) o nome desta função
pub extern "C" fn _start() -> ! {
    // essa função é o ponto de entrada, já que o vinculador procura uma função
    // denominado `_start` por padrão
    loop {}
}

/// Esta função é chamada em caso de pânico.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}