#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::os::raw::c_int;

// MARK: - Get and Set will be implemented together
impl Field {
    unsafe fn cell(&self, x: c_int, y: c_int) -> *mut bool {
        // If the x or y coordinates are outside the field boundaries they are wrapped
        // toroidally. For instance, an x value of -1 is treated as width-1.
        let x = x + self.w;
        let x = x % self.w;
        let y = y + self.h;
        let y = y % self.h;
        // Keep this from getting out of bounds
        unsafe {
            let ptr: *mut bool = (*self.s.add(x as usize)).add(y as usize);

            ptr
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Set(self_: &Field, x: c_int, y: c_int, b: bool) {
    unsafe { *self_.cell(x, y) = b }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Get(self_: &Field, x: c_int, y: c_int) -> bool {
    unsafe { *((*self_).cell(x, y)) }
}


