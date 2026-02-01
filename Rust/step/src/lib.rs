#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::ptr;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Step(self_: &mut Life) {
    unsafe {
        let a = &mut *self_.a;
        let b = &mut *self_.b;
        // Update the state of the next field (b) from the current field (a).
        for y in 0..self_.h {
            for x in 0..self_.w {
                Set(b, x, y, Next(a, x, y));
            }
        }
    }

	// Swap fields a and b.
    unsafe { ptr::swap(self_.a, self_.b); }
}

