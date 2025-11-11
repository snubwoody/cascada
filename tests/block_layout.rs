use cascada::{BlockLayout, EmptyLayout, IntrinsicSize, Layout, Size, solve_layout};

#[test]
fn respect_child_max_width() {
    let window = Size::new(800.0, 800.0);
    let child = EmptyLayout::new()
        .max_width(20.0)
        .intrinsic_size(IntrinsicSize::fill());

    let mut root = BlockLayout::new(child).intrinsic_size(IntrinsicSize::fill());

    solve_layout(&mut root, window);
    assert_eq!(root.children()[0].size().width, 20.0);
}

#[test]
fn fill_fixed_parent() {
    let window = Size::new(800.0, 800.0);
    let child = EmptyLayout::new()
        .intrinsic_size(IntrinsicSize::fill());

    let mut root = BlockLayout::new(child)
        .intrinsic_size(IntrinsicSize::fixed(100.0,50.0));

    solve_layout(&mut root, window);
    dbg!(&root);
    assert_eq!(root.children()[0].size().width, 100.0);
    assert_eq!(root.children()[0].size().height, 50.0);
}
