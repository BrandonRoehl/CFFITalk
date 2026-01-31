#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_int;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Field {
    fn cell<'a>(self: Field, x: c_int, y: c_int) -> Option<&'a mut bool> {
        // New index
        let i: c_int = x + y * self.w;
        // Keep this from getting out of bounds
        if i > 0 && self.w * self.h < i {
            unsafe {
                let ptr: *mut bool = self.s.add(i as usize);

                ptr.as_mut()
            }
        } else {
            None
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Set(self_: &mut Field, x: c_int, y: c_int, b: bool) {
    *self_.cell(x, y).expect("out of range") = b;
}

#[unsafe(no_mangle)]
pub extern "C" fn Get(self_: &mut Field, x: c_int, y: c_int) -> bool {
    *self_.cell(x, y).expect("out of range")
}


// unsafe extern "C" {
//     pub fn Next(self_: *mut Field, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> bool;
// }
// unsafe extern "C" {
//     pub fn Step(self_: *mut Life, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
// }
