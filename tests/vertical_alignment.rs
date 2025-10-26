use cascada::{
    AxisAlignment, EmptyLayout, IntrinsicSize, Layout, Padding, Position, Size, VerticalLayout,
    solve_layout,
};

#[test]
fn test_single_center_alignment() {
    let window = Size::new(500.0, 500.0);

    let child_1 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(250.0, 350.0));

    let mut root = VerticalLayout::new()
        .cross_axis_alignment(AxisAlignment::Center)
        .main_axis_alignment(AxisAlignment::Center)
        .padding(Padding::all(24.0))
        .intrinsic_size(IntrinsicSize::flex(1))
        .add_child(child_1);

    solve_layout(&mut root, window);

    let child_y = (root.size().height - root.children()[0].size().height) / 2.0 + root.position().y;
    let child_x = (root.size().width - root.children()[0].size().width) / 2.0 + root.position().x;

    assert_eq!(
        root.children()[0].position(),
        Position::new(child_x, child_y)
    );
}

#[test]
fn test_center_alignment() {
    let window = Size::new(1500.0, 1500.0);

    let child_1 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(250.0, 350.0));

    let child_2 = child_1.clone();
    let child_3 = child_1.clone();

    let mut root = VerticalLayout::new()
        .main_axis_alignment(AxisAlignment::Center)
        .cross_axis_alignment(AxisAlignment::Center)
        .padding(Padding::all(24.0))
        .spacing(50)
        .intrinsic_size(IntrinsicSize::fill())
        .add_child(child_1)
        .add_child(child_2)
        .add_child(child_3);

    solve_layout(&mut root, window);

    let height_sum = (350.0 * 3.0) + (50.0 * 2.0);
    let center_start = (root.size().height - height_sum) / 2.0;

    let child_1_pos = Position {
        x: (root.size().width - root.children()[0].size().width) / 2.0 + root.position().x,
        y: center_start,
    };

    let child_2_pos = Position {
        x: (root.size().width - root.children()[1].size().width) / 2.0 + root.position().x,
        y: center_start + root.children()[0].size().height + 50.0,
    };

    // A bit long but allow it
    let child_3_pos = Position {
        y: center_start
            + root.children()[0].size().height
            + root.children()[1].size().height
            + (50.0 * 2.0),
        x: (root.size().width - root.children()[2].size().width) / 2.0 + root.position().x,
    };

    assert_eq!(root.children()[0].position(), child_1_pos);
    assert_eq!(root.children()[1].position(), child_2_pos);
    assert_eq!(root.children()[2].position(), child_3_pos);
}

// TODO test overflow
