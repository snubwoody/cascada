
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

Cascada is a lightweight, high-performance UI layout engine.



[API Docs](https://docs.rs/cascada/latest/cascada) |
[Architecture](./ARCHITECTURE.md) 


## Features
- Microsecond layout performance
- Declarative syntax
- Predictable layouts
- Descriptive error handling

## Usage
Add this to your `Cargo.toml`.

```toml
[dependencies]
cascada = "0.2.0"
```

## Example

```rust
use cascada::{solve_layout,HorizontalLayout,EmptyLayout,Size};

let child = EmptyLayout::new()
    .intrinsic_size(IntrinsicSize::fill());

// Add three equally sized child nodes.
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

## Implementation details
Cascada was originally [agape_layout](https://crates.io/crates/agape_layout) which I made for 
[agape](https://crates.io/crates/agape), but I forked it because over time it started to feel 
like more of a standalone crate.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

