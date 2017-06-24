use test::Bencher;
use free_fns::*;

const NUM_ITER: usize = 100;

#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| 1)
}

#[bench]
fn v100w2(b: &mut Bencher) {
    let mut data = vec![0isize; 100];
    let mut buf = vec![0isize; 100];
    let w_size = 2;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v1000w2(b: &mut Bencher) {
    let mut data = vec![0isize; 1000];
    let mut buf = vec![0isize; 1000];
    let w_size = 2;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v10000w2(b: &mut Bencher) {
    let mut data = vec![0isize; 10000];
    let mut buf = vec![0isize; 10000];
    let w_size = 2;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v100000w2(b: &mut Bencher) {
    let mut data = vec![0isize; 100000];
    let mut buf = vec![0isize; 100000];
    let w_size = 2;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v100w2par(b: &mut Bencher) {
    let mut data = vec![0isize; 100];
    let mut buf = vec![0isize; 100];
    let w_size = 2;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v1000w2par(b: &mut Bencher) {
    let mut data = vec![0isize; 1000];
    let mut buf = vec![0isize; 1000];
    let w_size = 2;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v10000w2par(b: &mut Bencher) {
    let mut data = vec![0isize; 10000];
    let mut buf = vec![0isize; 10000];
    let w_size = 2;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v100000w2par(b: &mut Bencher) {
    let mut data = vec![0isize; 100000];
    let mut buf = vec![0isize; 100000];
    let w_size = 2;
    data[0] = 1;

    b.iter(|| for _ in 1..NUM_ITER {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
