use cascada::{AxisAlignment, EmptyLayout, HorizontalLayout, IntrinsicSize, Size, solve_layout};

fn main() {
    // Define the maximum available size
    let window_size = Size::unit(1000.0);
    let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(50.0, 50.0));

    let mut layout = HorizontalLayout::new()
        .add_child(child)
        .intrinsic_size(IntrinsicSize::fill()) // Make the layout fill the window
        .main_axis_alignment(AxisAlignment::Center) // Align the child in the center on the main axis
        .cross_axis_alignment(AxisAlignment::Center); // Align the child in the center on the cross axis

    solve_layout(&mut layout, window_size);
}
