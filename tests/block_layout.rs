use cascada::{solve_layout, BlockLayout, EmptyLayout, IntrinsicSize, Layout, Size};

#[test]
fn respect_child_max_width() {
    let window = Size::new(800.0, 800.0);
    let child = EmptyLayout::new()
        .max_width(20.0)
        .intrinsic_size(IntrinsicSize::fill());

    let mut root = BlockLayout::new(child)
        .intrinsic_size(IntrinsicSize::fill());

    solve_layout(&mut root, window);
    assert_eq!(root.children()[0].size().width, 20.0);
}