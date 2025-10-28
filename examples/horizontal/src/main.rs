use cascada::{AxisAlignment, EmptyLayout, HorizontalLayout, IntrinsicSize, Size, solve_layout};
use cascada::debug::DebugTree;

fn main() {
    // Define the maximum available size
    let window_size = Size::unit(1000.0);

    // Create three equally sized empty nodes
    let child1 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(50.0, 50.0));
    let child2 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(50.0, 50.0));
    let child3 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(50.0, 50.0));

    let mut layout = HorizontalLayout::new()
        .add_children([child1, child2, child3])
        .intrinsic_size(IntrinsicSize::fill()) // Make the layout fill the window
        .main_axis_alignment(AxisAlignment::Center)
        .cross_axis_alignment(AxisAlignment::Center);

    solve_layout(&mut layout, window_size);

    layout.debug_tree()
}
