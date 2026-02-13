#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use std::{
    fmt::{self, Display, Formatter},
    ptr, slice, thread, time,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Field {
    pub fn new(w: i32, h: i32) -> Self {
        let xyfield: Box<[bool]> = vec![false; (w * h) as usize].into_boxed_slice();
        let mut xfield: Box<[*mut bool]> = vec![ptr::null_mut(); w as usize].into_boxed_slice();

        let xyfirst = Box::leak(xyfield).as_mut_ptr();
        for x in 0..w as usize {
            xfield[x] = unsafe { xyfirst.add(x * h as usize) };
        }

        let s = Box::leak(xfield).as_mut_ptr();

        Field { w, h, s }
    }
}

impl Drop for Field {
    fn drop(&mut self) {
        unsafe {
            let xfield = slice::from_raw_parts_mut(self.s, self.w as usize);
            let yxfield = slice::from_raw_parts_mut(xfield[0], (self.w * self.h) as usize);

            // Drop both pieces of memory
            _ = Box::from_raw(yxfield);
            _ = Box::from_raw(xfield);
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

    pub fn randomize(&mut self) {
        unsafe {
            srand(
                time::SystemTime::now()
                    .duration_since(time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as u32,
            );
            for _ in 0..50 {
                let x = rand() % self.w;
                let y = rand() % self.h;
                Set(self.a, x, y, true);
            }
        }
    }
}

impl Drop for Life {
    fn drop(&mut self) {
        unsafe {
            // Drop the fields
            _ = Box::from_raw(self.a);
            _ = Box::from_raw(self.b);
        }
    }
}

impl Display for Life {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let a = unsafe { &*self.a };
        writeln!(f, "{}", a)?;
        Ok(())
    }
}

fn main() {
    println!("Conway's Game of Life");
    let life = &mut Life::new(40, 15);

    // Generate random starting field
    life.randomize();

    print!("\x1b7");
    for _ in 0..300 {
        unsafe { Step(life) };
        print!("\x1b8");
        println!("{}", life);
        thread::sleep(time::Duration::from_millis(1000 / 3));
    }
}
