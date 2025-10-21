use agape_layout::{
    BlockLayout, BoxSizing, EmptyLayout, HorizontalLayout, IntrinsicSize, Layout, Padding, Size,
    solve_layout,
};

#[test]
fn horizontal_layout() {
    let window = Size::new(800.0, 800.0);
    let mut root = HorizontalLayout::new();
    let mut child_1 = HorizontalLayout::new();
    let mut child_2 = HorizontalLayout::new();

    child_1.intrinsic_size.width = BoxSizing::Fixed(400.0);
    child_1.intrinsic_size.height = BoxSizing::Fixed(200.0);

    child_2.intrinsic_size.width = BoxSizing::Fixed(500.0);
    child_2.intrinsic_size.height = BoxSizing::Fixed(350.0);

    root.add_child(child_1);
    root.add_child(child_2);

    solve_layout(&mut root, window);

    assert_eq!(root.size(), Size::new(900.0, 350.0));

    assert_eq!(root.children()[0].size(), Size::new(400.0, 200.0));

    assert_eq!(root.children()[1].size(), Size::new(500.0, 350.0));
}

#[test]
fn horizontal_and_empty_layout() {
    let window = Size::new(1000.0, 1000.0);

    let mut child_1 = EmptyLayout::new();
    child_1.intrinsic_size.width = BoxSizing::Fixed(250.0);
    child_1.intrinsic_size.height = BoxSizing::Flex(1);

    let mut child_2 = EmptyLayout::new();
    child_2.intrinsic_size.width = BoxSizing::Flex(1);
    child_2.intrinsic_size.height = BoxSizing::Fixed(20.0);

    let mut child_3 = EmptyLayout::new();
    child_3.intrinsic_size.height = BoxSizing::Fixed(250.0);

    let mut root = HorizontalLayout::new();
    root.add_children([child_1, child_2, child_3]);

    solve_layout(&mut root, window);

    assert_eq!(root.size(), Size::new(250.0, 250.0));
    assert_eq!(root.children[0].size(), Size::new(250.0, 250.0));
    assert_eq!(root.children[1].size(), Size::new(0.0, 20.0));
    assert_eq!(root.children[2].size(), Size::new(0.0, 250.0));
}

#[test]
fn test_flex_sizing() {
    let window = Size::new(800.0, 800.0);
    let mut root = HorizontalLayout::new();
    let mut child_1 = HorizontalLayout::new();
    let mut child_2 = HorizontalLayout::new();

    child_1.intrinsic_size.width = BoxSizing::Flex(1);
    child_1.intrinsic_size.height = BoxSizing::Flex(1);

    child_2.intrinsic_size.width = BoxSizing::Flex(1);
    child_2.intrinsic_size.height = BoxSizing::Flex(1);

    root.intrinsic_size.width = BoxSizing::Flex(1);
    root.intrinsic_size.height = BoxSizing::Flex(1);

    root.add_child(child_1);
    root.add_child(child_2);

    solve_layout(&mut root, window);
    let child_size = Size::new(400.0, 800.0);
    assert_eq!(root.size(), window);
    assert_eq!(root.children()[0].size(), child_size);
    assert_eq!(root.children()[1].size(), child_size);
}

#[test]
fn flex_with_shrink() {
    let window = Size::new(800.0, 800.0);
    let padding = Padding::all(24.0);
    let spacing = 45;

    let inner_child = EmptyLayout {
        intrinsic_size: IntrinsicSize::fixed(250.0, 250.0),
        ..EmptyLayout::new()
    };

    let mut block = BlockLayout::new(Box::new(inner_child));
    block.padding = Padding::all(24.0);

    let empty = EmptyLayout {
        intrinsic_size: IntrinsicSize::flex(1),
        ..EmptyLayout::new()
    };

    let mut root = HorizontalLayout {
        padding,
        spacing,
        ..HorizontalLayout::new()
    };
    root.intrinsic_size.width = BoxSizing::Flex(1);
    root.add_child(block);
    root.add_child(empty);

    solve_layout(&mut root, window);

    let mut child_1_size = Size::new(250.0, 250.0);
    child_1_size.width += padding.horizontal_sum();
    child_1_size.height += padding.vertical_sum();

    let mut root_size = Size::new(window.width, child_1_size.height);
    root_size.height += padding.vertical_sum(); // Add the padding for child_1 and for the root

    let mut empty_size = Size::new(window.width, child_1_size.height);
    empty_size.width -= child_1_size.width;
    empty_size.width -= spacing as f32;
    empty_size.width -= padding.horizontal_sum();
    empty_size.height += padding.vertical_sum();

    let empty = &root.children[1];
    assert_eq!(empty.size(), empty_size);
    assert_eq!(root.children[0].size(), child_1_size);
    assert_eq!(root.size(), root_size);
}

#[test]
fn flex_with_fixed() {
    let window = Size::new(800.0, 800.0);
    let padding = Padding::all(24.0);
    let spacing = 45;

    let child_1 = EmptyLayout {
        intrinsic_size: IntrinsicSize::fixed(250.0, 250.0),
        ..Default::default()
    };

    let mut child_2 = EmptyLayout::new();
    child_2.intrinsic_size.width = BoxSizing::Flex(1);
    child_2.intrinsic_size.height = BoxSizing::Flex(2);

    let child_3 = EmptyLayout {
        intrinsic_size: IntrinsicSize::flex(4),
        ..Default::default()
    };

    let mut root = HorizontalLayout {
        padding,
        spacing,
        intrinsic_size: IntrinsicSize::fill(),
        ..Default::default()
    };
    root.add_children([child_1, child_2, child_3]);

    solve_layout(&mut root, window);

    let mut space = window;
    space.width -= spacing as f32 * 2.0;
    space.width -= padding.horizontal_sum();
    space.height -= padding.vertical_sum();
    space.width -= 250.0;

    assert_eq!(
        root.children[1].size().height,
        window.height - padding.vertical_sum()
    );
    assert_eq!(root.children[2].size().width, 4.0 / 5.0 * space.width);
    assert_eq!(root.children[1].size().width, 1.0 / 5.0 * space.width);
}

#[test]
fn test_flex_factor() {
    let window = Size::new(800.0, 400.0);
    let mut node = HorizontalLayout::new();
    let mut child_node_1 = HorizontalLayout::new();
    let mut child_node_2 = HorizontalLayout::new();

    child_node_1.intrinsic_size.width = BoxSizing::Flex(1);
    child_node_1.intrinsic_size.height = BoxSizing::Flex(1);

    child_node_2.intrinsic_size.width = BoxSizing::Flex(3);
    child_node_2.intrinsic_size.height = BoxSizing::Flex(3);

    node.intrinsic_size.width = BoxSizing::Flex(1);
    node.intrinsic_size.height = BoxSizing::Flex(1);

    node.add_child(child_node_1);
    node.add_child(child_node_2);

    solve_layout(&mut node, window);

    let flex_1_width = 1.0 / 4.0 * window.width;
    // The two children should both be half the size
    assert_eq!(node.children()[0].size().width, flex_1_width);
    assert_eq!(node.children()[0].size().height, 400.0);
    assert_eq!(
        node.children()[0].size().height,
        node.children()[1].size().height,
    );
    assert!(node.children()[1].size().width == 3.0 * node.children()[0].size().width);
    assert!(node.children()[1].size().height != 3.0 * node.children()[0].size().height);
}
