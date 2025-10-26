use cascada::{
    BoxSizing, EmptyLayout, IntrinsicSize, Layout, Padding, Size, VerticalLayout, solve_layout,
};

#[test]
fn subtract_fixed_height_from_available_space() {
    let inner_child = EmptyLayout::new().intrinsic_size(IntrinsicSize {
        height: BoxSizing::Fixed(250.0),
        ..Default::default()
    });

    let flex_child = EmptyLayout::new().intrinsic_size(IntrinsicSize {
        height: BoxSizing::Flex(1),
        ..Default::default()
    });

    let padding = Padding::all(10.0);
    let mut root = VerticalLayout::new()
        .spacing(24)
        .padding(padding)
        .add_children([inner_child.clone(), inner_child, flex_child])
        .intrinsic_size(IntrinsicSize {
            height: BoxSizing::Flex(1),
            ..Default::default()
        });

    solve_layout(&mut root, Size::new(1000.0, 1000.0));

    let mut flex_child_height = 1000.0;
    flex_child_height -= 250.0 * 2.0;
    flex_child_height -= 24.0 * 2.0;
    flex_child_height -= padding.vertical_sum();

    assert_eq!(root.children()[2].size().height, flex_child_height)
}
