
# Cascada
<div>
    <a href="https://crates.io/crates/cascada">
        <img alt="Crates.io Version" src="https://img.shields.io/crates/v/cascada">
    </a>
    <a href="https://docs.rs/cascada/latest/cascada/">
        <img src="https://img.shields.io/docsrs/cascada"/>
    </a>
    <img src="https://img.shields.io/github/actions/workflow/status/snubwoody/agape-rs/rust.yml"/>
    <a href="https://codecov.io/gh/snubwoody/cascada" > 
        <img src="https://codecov.io/gh/snubwoody/cascada/graph/badge.svg?token=PLYL0VUB5Y"/> 
    </a>
</div>

Cascada is a lightweight, high-performance UI layout engine. Cascada's goals are to be
fast **and** have an intuitive API, as well as overcoming the confusing aspect of CSS.


## Features
- Microsecond layout performance
- Declarative syntax
- Predictable layouts
- Descriptive error handling

## Usage
Add this to your `Cargo.toml`.

```toml
[dependencies]
cascada = "0.1.0"
```

## Example

```rust
use cascada::{solve_layout,HorizontalLayout,EmptyLayout,Size};

let child = EmptyLayout::new()
    .intrinsic_size(IntrinsicSize::fill());

// Add three equally sizes child nodes.
let mut layout = HorizontalLayout::new()
    .intrinsic_size(IntrinsicSize::fill())
    .add_child(child.clone())
    .add_child(child.clone())
    .add_child(child);

solve_layout(&mut layout, Size::unit(3000.0));

let children = layout.children();

assert_eq!(children[0].size().width,1000.0);
assert_eq!(children[1].size().width,1000.0);
assert_eq!(children[2].size().width,1000.0);
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

