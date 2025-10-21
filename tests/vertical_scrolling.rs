use agape_layout::{BoxSizing, EmptyLayout, Size, VerticalLayout, solve_layout};

#[test]
fn scrolling() {
    let mut child = EmptyLayout::new();
    child.intrinsic_size.height = BoxSizing::Fixed(200.0);
    let children = vec![
        child.clone(),
        child.clone(),
        child.clone(),
        child.clone(),
        child.clone(),
        child,
    ];

    let scroll_offset = 100.0;
    let mut root = VerticalLayout::new();
    root.intrinsic_size.height = BoxSizing::Fixed(200.0);
    root.scroll(scroll_offset);
    root.add_children(children);

    let window = Size::unit(400.0);

    solve_layout(&mut root, window);
    assert_eq!(root.children[0].position().y, scroll_offset);
}
