use criterion::{Criterion, criterion_group, criterion_main, BenchmarkId};
use cascada::{solve_layout, EmptyLayout, HorizontalLayout, IntrinsicSize, Size};

pub fn benchmark(c: &mut Criterion){
    let sizes = [1,10,100,1000,10000];
    let mut g = c.benchmark_group("nodes");
    for size in sizes{
        g.bench_with_input(BenchmarkId::from_parameter(size),&size,|b,size| {
            let child = EmptyLayout{
                intrinsic_size: IntrinsicSize::flex(1),
                ..EmptyLayout::default()
            };


            let mut layout = HorizontalLayout{
                intrinsic_size: IntrinsicSize::fill(),
                ..HorizontalLayout::default()
            };

            for _ in 0..*size{
                layout.add_child(child.clone());
            }

            b.iter(|| {
                solve_layout(&mut layout, Size::unit(1000.0))
            })
        });
    }

    g.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);