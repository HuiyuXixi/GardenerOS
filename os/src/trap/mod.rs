use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    stvec,
    scause::{self, Trap, Exception},
    stval,
};

mod context;
pub use context::TrapContext;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as *const () as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            // 暂时拦截系统调用，用于验证 Trap 是否生效
            panic!("Unsupported syscall: {}", cx.x[17]);
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
}
