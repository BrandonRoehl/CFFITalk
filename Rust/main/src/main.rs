#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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

fn main() {
    let mut life = Box::new(Life::new(10, 10));
    let lptr = life.as_mut();

    for _ in 0..10 {
        unsafe { Step(lptr) };
    }
    println!("Hello, world!");
}
