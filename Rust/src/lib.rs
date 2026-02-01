#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::os::raw::c_int;

// MARK: - Get and Set will be implemented together
impl Field {
    fn cell(self: &mut Field, x: c_int, y: c_int) -> &mut bool {
        // If the x or y coordinates are outside the field boundaries they are wrapped
        // toroidally. For instance, an x value of -1 is treated as width-1.
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
pub unsafe extern "C" fn Set(self_: &mut Field, x: c_int, y: c_int, b: bool) {
    *self_.cell(x, y) = b
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Get(self_: &mut Field, x: c_int, y: c_int) -> bool {
    *self_.cell(x, y)
}

// MARK: - Next state of a specific cell

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Next(self_: &mut Field, x: c_int, y: c_int) -> bool {
	// Count the adjacent cells that are alive.
	let mut alive = 0;
	for i in -1..=1 {
	    for j in -1..=1 {
			if (j != 0 || i != 0) && unsafe { Get(self_, x+i, y+j) } {
				alive += 1;
			}
		}
	}
	// Return next state according to the game rules:
	//   exactly 3 neighbors: on,
	//   exactly 2 neighbors: maintain current state,
	//   otherwise: off.
	alive == 3 || alive == 2 && unsafe { Get(self_, x, y) }
}

// MARK: - Increment the game

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Step(self_: &mut Life) {
}

