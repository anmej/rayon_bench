extern crate mylib;
use mylib::free_fns::*;

fn main() {
    let mut data = vec![0isize; 10];
    let mut buf = vec![0isize; 10];
    let w_size = 2;
    data[0] = 1;
    for _ in 1..4 {
        step_iter(&mut data, &mut buf, 3);
        println!("{:?}", data);
    }

    let mut data = vec![0isize; 10];
    let mut buf = vec![0isize; 10];
    let w_size = 2;
    data[0] = 1;
    for _ in 1..4 {
        step_par_iter(&mut data, &mut buf, 3);
        println!("{:?}", data);
    }
}
