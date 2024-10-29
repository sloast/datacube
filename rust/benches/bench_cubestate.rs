use criterion::{black_box, criterion_group, criterion_main, Criterion};
use datacube::cube::{Facelets, Face, Cube};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut cube = Facelets::new();
    c.bench_function("rotate_inplace", |b| b.iter(|| cube.rotate(Face::R,black_box(1))));
    c.bench_function("rotate_copy", |b| b.iter(|| cube = cube.rotated(Face::R,black_box(1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

