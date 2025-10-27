use cascada::{solve_layout, BoxSizing, EmptyLayout, HorizontalLayout, IntrinsicSize, Layout, Padding, Size, VerticalLayout};

// TODO: add debug_tree
fn navbar() -> HorizontalLayout{
    let panel = EmptyLayout::new()
        .intrinsic_size(IntrinsicSize{
            width: BoxSizing::Fixed(50.0),
            height: BoxSizing::Flex(1),
        });

    let panels = (0..2).map(|_|panel.clone());

    HorizontalLayout::new()
        .add_children(panels)
        .padding(Padding::symmetric(12.0,24.0))
}

fn sidebar() -> VerticalLayout {
    let size = IntrinsicSize{
        width: BoxSizing::Fixed(250.0),
        height: BoxSizing::Flex(1),
    };

    let panel = EmptyLayout::new()
        .intrinsic_size(IntrinsicSize{
            width: BoxSizing::Flex(1),
            height: BoxSizing::Fixed(50.0),
        });

    let panels = (0..5).map(|_|panel.clone());

    VerticalLayout::new()
        .intrinsic_size(size)
        .spacing(12)
        .padding(Padding::all(20.0))
        .add_children(panels)
}

fn section() -> VerticalLayout {
    let cell = EmptyLayout::new()
        .intrinsic_size(IntrinsicSize::fill());

    let row1 = HorizontalLayout::new()
        .add_children([cell.clone(),cell.clone(),cell.clone()]);

    let row2 = HorizontalLayout::new()
        .add_children([cell.clone(),cell.clone(),cell]);

    VerticalLayout::new()
        .padding(Padding::all(24.0))
        .spacing(16)
        .add_children([row1,row2])
        .intrinsic_size(IntrinsicSize::fill())
}

fn main() {
    let window_size = Size::unit(1920.0);

    let body = HorizontalLayout::new()
        .add_child(sidebar())
        .add_child(section());

    let mut page = VerticalLayout::new()
        .add_children([navbar(),body]);

    solve_layout(&mut page,window_size);
}
