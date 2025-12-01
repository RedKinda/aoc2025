use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let inp = std::fs::read_to_string(format!("input/{}.txt", aoc::DAY)).unwrap();
    let bench_name = format!("Day {}{}", aoc::DAY, aoc::PART);
    c.bench_function(&bench_name, |b| b.iter(|| aoc::run(&inp)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
