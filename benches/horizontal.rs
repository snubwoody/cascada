use cascada::{EmptyLayout, HorizontalLayout, IntrinsicSize, Size, solve_layout};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

pub fn benchmark(c: &mut Criterion) {
    let sizes = [1, 10, 100, 1000, 10000];
    let mut g = c.benchmark_group("nodes");
    for size in sizes {
        g.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, size| {
            let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fill()); 
            let mut layout = HorizontalLayout {
                intrinsic_size: IntrinsicSize::fill(),
                ..HorizontalLayout::default()
            };

            for _ in 0..*size {
                layout.add_child(child.clone());
            }

            b.iter(|| solve_layout(&mut layout, Size::unit(1000.0)))
        });
    }

    g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
