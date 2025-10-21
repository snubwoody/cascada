use agape_layout::{
    BlockLayout, BoxSizing, EmptyLayout, IntrinsicSize, Padding, Size, VerticalLayout, solve_layout,
};

// TODO test than fill sizing is working with shrink elements
#[test]
fn child_flex_height_works_with_shrink_height() {
    let inner_child = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            height: BoxSizing::Fixed(250.0),
            ..Default::default()
        },
        ..Default::default()
    };

    let flex_child = EmptyLayout {
        intrinsic_size: IntrinsicSize {
            height: BoxSizing::Flex(1),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut root = VerticalLayout {
        spacing: 24,
        padding: Padding::all(10.0),
        children: vec![
            Box::new(BlockLayout::new(Box::new(inner_child.clone()))),
            Box::new(flex_child),
            Box::new(BlockLayout::new(Box::new(inner_child))),
        ],
        intrinsic_size: IntrinsicSize {
            height: BoxSizing::Flex(1),
            ..Default::default()
        },
        ..Default::default()
    };

    solve_layout(&mut root, Size::new(1000.0, 1000.0));

    let mut flex_child_height = 1000.0;
    flex_child_height -= 250.0 * 2.0;
    flex_child_height -= 24.0 * 2.0;
    flex_child_height -= 10.0 * 2.0;

    assert_eq!(root.children[1].size().height, flex_child_height)
}
