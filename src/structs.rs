use rayon::prelude::*;
use std::cmp::{min, max};

pub struct Foo1D {
    pub data: Vec<isize>,
    pub buf: Vec<isize>,
}

impl Foo1D {
    pub fn new(len: usize) -> Foo1D {
        Foo1D {
            data: vec![0; len],
            buf: vec![0; len],
        }
    }

    pub fn step(&mut self, window_size: usize) {
        let w_size = window_size as isize;
        for i in 0..self.data.len() as isize {
            let w_start: usize = max(0, i - w_size / 2) as usize;
            let w_end: usize = min(self.data.len() as isize, i + w_size / 2 + w_size % 2) as usize;
            //let s = &v[w_start..w_end];
            let w_sum: isize = self.data[w_start..w_end].iter().sum();
            self.buf[i as usize] = w_sum;

        }
        use std::mem::swap;
        swap(&mut self.data, &mut self.buf);
        println!("{:?}", self.data);
    }

    pub fn step_iter_data(&mut self, window_size: usize) {
        let w_size = window_size as isize;
        for (i, n) in self.data.iter().enumerate() {
            let i: isize = i as isize;
            let w_start: usize = max(0, i - w_size / 2) as usize;
            let w_end: usize = min(self.data.len() as isize, i + w_size / 2 + w_size % 2) as usize;
            let w_sum: isize = self.data[w_start..w_end].iter().sum();
            self.buf[i as usize] = w_sum;
        }
        use std::mem::swap;
        swap(&mut self.data, &mut self.buf);
        println!("{:?}", self.data);
    }

    pub fn step_iter_buf(&mut self, window_size: usize) {
        let w_size = window_size as isize;
        let len = self.buf.len();
        for (i, n) in self.buf.iter_mut().enumerate() {
            let i: isize = i as isize;
            let w_start: usize = max(0, i - w_size / 2) as usize;
            let w_end: usize = min(len as isize, i + w_size / 2 + w_size % 2) as usize;
            let w_sum: isize = self.data[w_start..w_end].iter().sum();
            *n = w_sum;
        }
        use std::mem::swap;
        swap(&mut self.data, &mut self.buf);
        println!("{:?}", self.data);
    }

    // pub fn step_par_iter_buf(&mut self, window_size: usize) {
    //     let w_size = window_size as isize;
    //     let len = self.buf.len();
    //     self.buf.par_iter_mut().enumerate().for_each(|(i, n)|{
    //         let i: isize = i as isize;
    //         let w_start: usize = max(0, i - w_size / 2) as usize;
    //         let w_end: usize = min(len as isize, i + w_size / 2 + w_size % 2) as
    //             usize;
    //         let w_sum: isize = self.data[w_start..w_end].iter().sum();
    //         *n = w_sum;
    //     });
    //     use std::mem::swap;
    //     swap(&mut self.data, &mut self.buf);
    //     println!("{:?}", self.data);
    // }
}
