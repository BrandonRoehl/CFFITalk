#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::os::raw::c_int;

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

