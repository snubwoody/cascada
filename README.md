# agape_layout

This is the crate that manages all the agape layouts, at a basic level every layout
node must return a size and position so that other layouts can arrange themselves
accordingly.

## Potential names

- Cascada

## Getting started

```rust
use agape_layout::{EmptyLayout, HorizontalLayout, LayoutSolver, Size};

let mut root = HorizontalLayout::new();
root.add_children([EmptyLayout::new(), EmptyLayout::new()]);

// Pass in the root layout and the window size
// The layout solver returns any errors that occured, such as layout overflow
let _ = LayoutSolver::solve( & mut root,Size::new(500.0, 500.0));
```

This layout engine is based on the idea that a [`Layout`] can only have one of three
different intrinsic sizes, known as [`BoxSizing`]

- It wants to be as large as possible, usually filling the parent, this is the
  `BoxSizing::Flex(u8)` variant. It also has a flex factor which can be used to
  control how much space it takes relative it's to sibling `Layouts`.
- It wants to be as small as possible, usually fitting it's children, this is the
  `BoxSizing::Shrink` variant
- It wants to be a certain fixed size, this is the `BoxSizing::Fixed` variant.

`agape_layout` uses `layouts` to perform calculations, a layout is anything which implements
the [`Layout`] trait. Currently there are 4 distinct types of [`Layout`]

- [`HorizontalLayout`]: Arranges children horizontally
- [`VerticalLayout`]: Arranges children verically
- [`BlockLayout`]: A layout with a single child
- [`EmptyLayout`]: A layout with no children, commonly used for things like
  text and images.

## Error handling

Errors are non-blocking, an error occuring for one `Layout` usually doesn't mean so everything else should halt so each
`Layout` keeps an error stack that can be fetched from the root `Layout`. This way trivial errors like `overflow` and
`out-of-bounds` can still be reported while the rest of the system continues. This also however means that if a parent
experienced an error then the children will be affected as well.

## TODO
- Add benchmarks

## License