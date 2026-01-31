#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_int;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// MARK: - Get and Set will be implemented together
impl Field {
    fn cell<'a>(self: &'a mut Field, x: c_int, y: c_int) -> &'a mut bool {
        let x = x % self.w;
        let y = y % self.h;
        // New index
        let i: c_int = x + y * self.w;
        // Keep this from getting out of bounds
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

// MARK: - Next state of a specific cell
pub extern "C" fn Next(self_: &mut Field, x: c_int, y: c_int) -> bool {
	// Count the adjacent cells that are alive.
	let mut alive = 0;
	for i in -1..=1 {
	    for j in -1..=1 {
			if (j != 0 || i != 0) && *self_.cell(x+i, y+j) {
				alive += 1;
			}
		}
	}
	// Return next state according to the game rules:
	//   exactly 3 neighbors: on,
	//   exactly 2 neighbors: maintain current state,
	//   otherwise: off.
	alive == 3 || alive == 2 && *self_.cell(x, y)
}

// MARK: - Increment the game

// unsafe extern "C" {
//     pub fn Step(self_: *mut Life, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
// }
