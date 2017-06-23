use test::Bencher;
use free_fns::*;

#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| 1)
}

#[bench]
fn v1000w1000(b: &mut Bencher) {
    let mut data = vec![0isize; 1000];
    let mut buf = vec![0isize; 1000];
    let w_size = 1000;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v10000w1000(b: &mut Bencher) {
    let mut data = vec![0isize; 10000];
    let mut buf = vec![0isize; 10000];
    let w_size = 1000;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v100000w1000(b: &mut Bencher) {
    let mut data = vec![0isize; 100000];
    let mut buf = vec![0isize; 100000];
    let w_size = 1000;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_iter(&mut data, &mut buf, w_size);
    })
}

#[bench]
fn v1000w1000par(b: &mut Bencher) {
    let mut data = vec![0isize; 1000];
    let mut buf = vec![0isize; 1000];
    let w_size = 1000;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v10000w1000par(b: &mut Bencher) {
    let mut data = vec![0isize; 10000];
    let mut buf = vec![0isize; 10000];
    let w_size = 1000;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v100000w1000par(b: &mut Bencher) {
    let mut data = vec![0isize; 100000];
    let mut buf = vec![0isize; 100000];
    let w_size = 1000;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
