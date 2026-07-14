use core::arch::asm;

pub fn hlt_loop() {
    loop {
        unsafe { asm!("hlt") }
    }
}

pub fn no_int_runner<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let eflags: u32;
    unsafe {
        asm!("pushfd", "pop {0}", out(reg) eflags, options(nomem, nostack));
    }

    let int_up = (eflags & (1 << 9)) != 0;

    if int_up {
        unsafe {
            asm!("cli", options(nomem, nostack));
        }
    }

    let result = f();

    if int_up {
        unsafe {
            asm!("sti", options(nomem, nostack));
        }
    }

    result
}
