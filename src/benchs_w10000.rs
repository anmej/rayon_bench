use test::Bencher;
use free_fns::*;

const NUM_ITER: usize = 100;

#[bench]
fn v10000w10000(b: &mut Bencher) {
    let mut data = vec![0isize; 10000];
    let mut buf = vec![0isize; 10000];
    let w_size = 10000;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v100000w10000(b: &mut Bencher) {
    let mut data = vec![0isize; 100000];
    let mut buf = vec![0isize; 100000];
    let w_size = 10000;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_iter(&mut data, &mut buf, w_size);
    })
}

#[bench]
fn v10000w10000par(b: &mut Bencher) {
    let mut data = vec![0isize; 10000];
    let mut buf = vec![0isize; 10000];
    let w_size = 10000;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v100000w10000par(b: &mut Bencher) {
    let mut data = vec![0isize; 100000];
    let mut buf = vec![0isize; 100000];
    let w_size = 10000;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
