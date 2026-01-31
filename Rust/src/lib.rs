#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_int;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// MARK: - Get and Set will be implemented together
impl Field {
    fn cell<'a>(self: Field, x: c_int, y: c_int) -> &'a mut bool {
        // New index
        let i: c_int = x + y * self.w;
        // Keep this from getting out of bounds
        assert!(i >= 0, "index negative");
        assert!(self.w * self.h < i, "index out of range");
        unsafe {
            let ptr: *mut bool = self.s.add(i as usize);

            ptr.as_mut().expect("null pointer")
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Set(self_: &mut Field, x: c_int, y: c_int, b: bool) {
    *self_.cell(x, y) = b
}

#[unsafe(no_mangle)]
pub extern "C" fn Get(self_: &mut Field, x: c_int, y: c_int) -> bool {
    *self_.cell(x, y)
}

//MARK: - Next state of a specific cell

// unsafe extern "C" {
//     pub fn Next(self_: *mut Field, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> bool;
// }

//MARK: - Increment the game

// unsafe extern "C" {
//     pub fn Step(self_: *mut Life, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
// }
