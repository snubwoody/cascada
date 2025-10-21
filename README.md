# Cascada

A simple, high performance, general purpose UI layout engine.
<div>
    <a href="https://codecov.io/gh/snubwoody/cascada" > 
     <img src="https://codecov.io/gh/snubwoody/cascada/graph/badge.svg?token=PLYL0VUB5Y"/> 
    </a>
</div>

## Goals

- Fast
- Simple
- Portable

## Installation
Add `cascada` to your project

```toml
[dependencies]
cascada = "0.1.0"
```

## Layout
This is a two pass layout algorithm, the minimum constraints flow up and the maximum constraints
flow down.

## Constraints
Constraints define the minimum and maximum size a layout node can be.

## Padding 
Padding is the space between the edges of a layout node and its content, the padding struct
has 4 sides: `left`, `right`, `top` and `bottom`.

## Axes
Each node has two axes: the main axis and the cross axis. The main axis is the axis which content
flow along and the cross axis is the axis perpendicular to the cross axis.

## Intrinsic size
Intrinsic size is the size that a layout node wants to be

For example to have two equally sized nodes in a horizontal node you would give them an intrinsic
width of `Flex`.

## Comparison to CSS
- Justify between: There is no justify between, you may use nodes in between and set the flex to 1.
## License

Licensed under either of:

Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## TODO

- Add benchmarks