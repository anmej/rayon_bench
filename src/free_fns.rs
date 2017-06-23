use rayon::prelude::*;
use std::cmp::{min, max};
use std::mem;

pub fn step_iter(data: &mut Vec<isize>, buf: &mut Vec<isize>, window_size: usize) {
    //assert_eq!(data.len(), buf.len());
    let w_size = window_size as isize;
    let len = data.len();
    for (i, n) in buf.iter_mut().enumerate() {
        let i: isize = i as isize;
        let w_start: usize = max(0, i - w_size / 2) as usize;
        let w_end: usize = min(len as isize, i + w_size / 2 + w_size % 2) as usize;
        let w_sum: isize = data[w_start..w_end].iter().sum();
        *n = w_sum;
    }
    mem::swap(data, buf);
    // println!("{:?}", self.data);
}

pub fn step_par_iter(data: &mut Vec<isize>, buf: &mut Vec<isize>, window_size: usize) {
    //assert_eq!(data.len(), buf.len());
    let w_size = window_size as isize;
    let len = data.len();
    buf.par_iter_mut().enumerate().for_each(|(i, n)| {
        let i: isize = i as isize;
        let w_start: usize = max(0, i - w_size / 2) as usize;
        let w_end: usize = min(len as isize, i + w_size / 2 + w_size % 2) as usize;
        let w_sum: isize = data[w_start..w_end].iter().sum();
        *n = w_sum;
    });
    mem::swap(data, buf);
    // println!("{:?}", buf);
}
