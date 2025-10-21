use agape_layout::{
    AxisAlignment, BoxSizing, EmptyLayout, HorizontalLayout, IntrinsicSize, Padding, Position,
    Size, solve_layout,
};

#[test]
fn single_horizontal_center_alignment() {
    let window = Size::new(500.0, 500.0);

    let child_1 = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Fixed(250.0),
            height: BoxSizing::Fixed(350.0),
        },
        ..Default::default()
    };

    let mut root = HorizontalLayout {
        main_axis_alignment: AxisAlignment::Center,
        cross_axis_alignment: AxisAlignment::Center,
        padding: Padding::all(24.0),
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Flex(1),
            height: BoxSizing::Flex(1),
        },
        ..Default::default()
    };
    root.add_child(child_1);

    solve_layout(&mut root, window);

    let child_y = (root.size.height - root.children[0].size().height) / 2.0 + root.position.y;
    let child_x = (root.size.width - root.children[0].size().width) / 2.0 + root.position.x;

    assert_eq!(root.children[0].position(), Position::new(child_x, child_y));
}

#[test]
fn horizontal_center_alignment() {
    let window = Size::new(1500.0, 1500.0);

    let child_1 = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Fixed(250.0),
            height: BoxSizing::Fixed(350.0),
        },
        ..Default::default()
    };

    let child_2 = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Fixed(250.0),
            height: BoxSizing::Fixed(350.0),
        },
        ..Default::default()
    };

    let child_3 = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Fixed(250.0),
            height: BoxSizing::Fixed(350.0),
        },
        ..Default::default()
    };

    let mut root = HorizontalLayout {
        main_axis_alignment: AxisAlignment::Center,
        cross_axis_alignment: AxisAlignment::Center,
        padding: Padding::all(24.0),
        spacing: 50,
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Flex(1),
            height: BoxSizing::Flex(1),
        },
        ..Default::default()
    };
    root.add_children([child_1, child_2, child_3]);

    solve_layout(&mut root, window);

    let width_sum = (250.0 * 3.0) + (50.0 * 2.0);
    let center_start = (root.size.width - width_sum) / 2.0;

    let child_1_pos = Position {
        x: center_start,
        y: (root.size.height - root.children[0].size().height) / 2.0 + root.position.y,
    };

    let child_2_pos = Position {
        x: center_start + root.children[0].size().width + 50.0,
        y: (root.size.height - root.children[1].size().height) / 2.0 + root.position.y,
    };

    // A bit long but allow it
    let child_3_pos = Position {
        x: center_start
            + root.children[0].size().width
            + root.children[1].size().width
            + (50.0 * 2.0),
        y: (root.size.height - root.children[2].size().height) / 2.0 + root.position.y,
    };

    assert_eq!(root.children[0].position(), child_1_pos);
    assert_eq!(root.children[1].position(), child_2_pos);
    assert_eq!(root.children[2].position(), child_3_pos);
}

#[test]
fn start_alignment() {
    let window = Size::new(200.0, 200.0);

    let padding = Padding::all(24.0);
    let spacing = 10;

    let child_1 = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Fixed(240.0),
            height: BoxSizing::Fixed(40.0),
        },
        ..Default::default()
    };

    let child_2 = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            width: BoxSizing::Fixed(20.0),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut root = HorizontalLayout {
        position: Position { x: 250.0, y: 10.0 },
        spacing,
        padding,
        children: vec![Box::new(child_1), Box::new(child_2)],
        ..Default::default()
    };

    solve_layout(&mut root, window);

    let mut child_1_pos = root.position;
    child_1_pos.x += padding.left;
    child_1_pos.y += padding.top;
    let mut child_2_pos = child_1_pos;
    child_2_pos.x += root.children[0].size().width + spacing as f32;

    assert_eq!(root.children[0].position(), child_1_pos);
    assert_eq!(root.children[1].position(), child_2_pos);
}

// TODO test overflow
