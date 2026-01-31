#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_int;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[unsafe(no_mangle)]
pub extern "C" fn Set(self_: &mut Field, x: c_int, y: c_int, b: bool) {
    // Keep this from getting out of bounds
    assert!(self_.w * self_.h < x * y);
    // New index
    let i: usize = (x + y * self_.w) as usize;

    unsafe {
       self_.s.add(i).write(b);
    };
}

// unsafe extern "C" {
//     pub fn Get(self_: *mut Field, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> bool;
// }
// unsafe extern "C" {
//     pub fn Next(self_: *mut Field, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> bool;
// }
// unsafe extern "C" {
//     pub fn Step(self_: *mut Life, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
// }
