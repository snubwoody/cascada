use agape_layout::{
    AxisAlignment, BlockLayout, BoxSizing, EmptyLayout, IntrinsicSize, Padding, Position, Size,
    solve_layout,
};

#[test]
fn center_alignment() {
    let window = Size::new(500.0, 500.0);

    let child = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Fixed(250.0),
            height: BoxSizing::Fixed(350.0),
        },
        ..Default::default()
    };

    let mut root = BlockLayout::new(Box::new(child));
    root.main_axis_alignment = AxisAlignment::Center;
    root.cross_axis_alignment = AxisAlignment::Center;
    root.padding = Padding::all(24.0);
    root.intrinsic_size.width = BoxSizing::Flex(1);
    root.intrinsic_size.height = BoxSizing::Flex(1);

    solve_layout(&mut root, window);

    let child_y = (root.size.height - root.child.size().height) / 2.0 + root.position.y;
    let child_x = (root.size.width - root.child.size().width) / 2.0 + root.position.x;

    assert_eq!(root.child.position(), Position::new(child_x, child_y));
}

#[test]
fn test_start_alignment() {
    let window = Size::new(200.0, 200.0);

    let padding = 32;

    let child_1 = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Fixed(240.0),
            height: BoxSizing::Fixed(40.0),
        },
        ..Default::default()
    };

    let mut root = BlockLayout::new(Box::new(child_1));
    root.position = Position::new(20.0, 500.0);
    root.padding = Padding::all(32.0);

    solve_layout(&mut root, window);

    let mut child_1_pos = root.position;
    child_1_pos += padding as f32;

    assert_eq!(root.child.position(), child_1_pos);
}

#[test]
fn test_end_alignment() {
    let window = Size::new(200.0, 200.0);

    let padding = 32;

    let child_1 = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Fixed(240.0),
            height: BoxSizing::Fixed(40.0),
        },
        ..Default::default()
    };

    let mut root = BlockLayout::new(Box::new(child_1));
    root.position = Position::new(250.0, 10.0);
    root.padding = Padding::all(32.0);
    root.main_axis_alignment = AxisAlignment::End;
    root.cross_axis_alignment = AxisAlignment::End;

    solve_layout(&mut root, window);

    let mut child_1_pos = Position {
        x: root.position.x + root.size.width,
        y: root.position.y + root.size.height,
    };
    child_1_pos -= padding as f32;

    assert_eq!(root.child.position(), child_1_pos);
}

// TODO test overflow
