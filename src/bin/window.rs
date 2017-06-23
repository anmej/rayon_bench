extern crate mylib;
use mylib::structs::*;
use mylib::free_fns::*;

fn main() {
    let mut f = Foo1D::new(10);
    f.data[1] = 1;
    for _ in 1..4 {
        step_iter(&mut f.data, &mut f.buf, 3);
        println!("{:?}", f.data);
    }

    let mut f = Foo1D::new(10);
    f.data[1] = 1;
    for _ in 1..4 {
        step_par_iter(&mut f.data, &mut f.buf, 3);
        println!("{:?}", f.data);
    }
}
