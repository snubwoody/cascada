use cascada::{
    AxisAlignment, BlockLayout, EmptyLayout, IntrinsicSize, Layout, Padding, Position, Size,
    solve_layout,
};

#[test]
fn center_alignment() {
    let window = Size::new(500.0, 500.0);

    let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(250.0, 350.0));
    let mut root = BlockLayout::new(Box::new(child))
        .main_axis_alignment(AxisAlignment::Center)
        .cross_axis_alignment(AxisAlignment::Center)
        .padding(Padding::all(24.0))
        .intrinsic_size(IntrinsicSize::flex(1));

    solve_layout(&mut root, window);

    let child_y = (root.size().height - root.child().size().height) / 2.0 + root.position().y;
    let child_x = (root.size().width - root.child().size().width) / 2.0 + root.position().x;

    assert_eq!(root.child().position(), Position::new(child_x, child_y));
}

#[test]
fn test_start_alignment() {
    let window = Size::new(200.0, 200.0);

    let padding = 32;

    let child_1 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(240.0, 40.0));
    let mut root = BlockLayout::new(Box::new(child_1)).padding(Padding::all(32.0));
    root.set_position(Position::new(20.0, 500.0));

    solve_layout(&mut root, window);

    let mut child_1_pos = root.position();
    child_1_pos += padding as f32;

    assert_eq!(root.child().position(), child_1_pos);
}

#[test]
fn test_end_alignment() {
    let window = Size::new(200.0, 200.0);

    let padding = 32;

    let child_1 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(240.0, 40.0));
    let mut root = BlockLayout::new(Box::new(child_1))
        .padding(Padding::all(32.0))
        .main_axis_alignment(AxisAlignment::End)
        .cross_axis_alignment(AxisAlignment::End);
    root.set_position(Position::new(250.0, 10.0));

    solve_layout(&mut root, window);

    let mut child_1_pos = Position {
        x: root.position().x + root.size().width,
        y: root.position().y + root.size().height,
    };
    child_1_pos -= padding as f32;

    assert_eq!(root.child().position(), child_1_pos);
}

// TODO test overflow
