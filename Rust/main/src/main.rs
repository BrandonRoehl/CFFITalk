#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{
    fmt::{Display, Formatter},
    thread,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Field {
    pub fn new(w: i32, h: i32) -> Self {
        let mut xyfield: Box<[bool]> = vec![false; (w * h) as usize].into_boxed_slice();

        let mut xfield: Box<[*mut bool]> =
            vec![std::ptr::null_mut(); w as usize].into_boxed_slice();

        let xyfirst = xyfield.as_mut_ptr();
        for x in 0..w as usize {
            xfield[x] = unsafe { xyfirst.add(x * h as usize) };
        }

        let s = xfield.as_mut_ptr();
        std::mem::forget(xyfield);
        std::mem::forget(xfield);

        Field { w, h, s }
    }
}

impl Drop for Field {
    fn drop(&mut self) {
        unsafe {
            let xfield = std::slice::from_raw_parts_mut(self.s, self.w as usize);
            let yxfield = std::slice::from_raw_parts_mut(xfield[0], (self.w * self.h) as usize);

            // Drop both pieces of memory
            Box::from_raw(yxfield);
            Box::from_raw(xfield);
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                let c = if unsafe { Get(self, x, y) } { '*' } else { ' ' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Life {
    pub fn new(w: i32, h: i32) -> Self {
        let a = Box::into_raw(Box::new(Field::new(w, h)));
        let b = Box::into_raw(Box::new(Field::new(w, h)));

        Life { a, b, h, w }
    }
}

impl Drop for Life {
    fn drop(&mut self) {
        unsafe {
            // Drop the fields
            Box::from_raw(self.a);
            Box::from_raw(self.b);
        }
    }
}

impl Display for Life {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let a = unsafe { &*self.a };
        writeln!(f, "{}", a)?;
        Ok(())
    }
}

fn main() {
    let mut life = Box::new(Life::new(40, 15));
    let lptr = life.as_mut();

    for _ in 0..50 {
        unsafe {
            let x = rand() % lptr.w;
            let y = rand() % lptr.h;
            Set(lptr.a, x, y, true);
        }
    }

    for _ in 0..300 {
        unsafe { Step(lptr) };
        println!("{}", lptr);
        thread::sleep(std::time::Duration::from_millis(50));
    }
}
