use cascada::debug::DebugTree;
use cascada::{
    AxisAlignment, BlockLayout, EmptyLayout, HorizontalLayout, IntrinsicSize, Padding, Size,
    solve_layout,
};

fn main() {
    let window_size = Size::unit(1000.0);

    let inner = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(150.0, 150.0));

    let child1 = BlockLayout::new(inner).padding(Padding::all(20.0));

    let child2 = EmptyLayout::new().intrinsic_size(IntrinsicSize::flex(2));
    let child3 = EmptyLayout::new().intrinsic_size(IntrinsicSize::flex(1));

    let mut layout = HorizontalLayout::new()
        .add_child(child1)
        .add_child(child2)
        .add_child(child3)
        .padding(Padding::all(20.0))
        .intrinsic_size(IntrinsicSize::fill());

    solve_layout(&mut layout, window_size);

    layout.debug_tree()
}
