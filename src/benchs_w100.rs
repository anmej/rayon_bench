use test::Bencher;
use free_fns::*;

#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| 1)
}

#[bench]
fn v1000w100(b: &mut Bencher) {
    let mut data = vec![0isize; 1000];
    let mut buf = vec![0isize; 1000];
    let w_size = 100;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v10000w100(b: &mut Bencher) {
    let mut data = vec![0isize; 10000];
    let mut buf = vec![0isize; 10000];
    let w_size = 100;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v100000w100(b: &mut Bencher) {
    let mut data = vec![0isize; 100000];
    let mut buf = vec![0isize; 100000];
    let w_size = 100;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_iter(&mut data, &mut buf, w_size);
    })
}

#[bench]
fn v1000w100par(b: &mut Bencher) {
    let mut data = vec![0isize; 1000];
    let mut buf = vec![0isize; 1000];
    let w_size = 100;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v10000w100par(b: &mut Bencher) {
    let mut data = vec![0isize; 10000];
    let mut buf = vec![0isize; 10000];
    let w_size = 100;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
#[bench]
fn v100000w100par(b: &mut Bencher) {
    let mut data = vec![0isize; 100000];
    let mut buf = vec![0isize; 100000];
    let w_size = 100;
    data[0] = 1;

    b.iter(|| for _ in 1..10 {
        step_par_iter(&mut data, &mut buf, w_size);
    })
}
