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

### Alignment
There are three `AxisAlignment` variants that specify how a node should align its children i.e.

- `AxisAlignment::Start`: Align content at the start of the axis.
- `AxisAlignment::Center`: Align content in the center of the axis.
- `AxisAlignement::End`: Align content at the end of the axis.

```
|----------------------------|
|                            |
|                            |
|                          | |
|                          | |
|____________________________|
```

TODO: Add figma diagrams

## Layouts

### Horizontal layout
This is a layout node that arranges it's content along the x-axis.

### Vertical layout

### Block layout

### Empty layout
A layout node with no children. The distinction between no children, one child and multiple children
is important, which is why they are separate. This is usually used for graphical elements such as 
text, images, icons and so on. Due to the fact that they have no children, internally, empty layouts
get to skip a lot of the calculations.

## Intrinsic size
Intrinsic size is the size that a layout node wants to be

For example to have two equally sized nodes in a horizontal node you would give them an intrinsic
width of `Flex`.

### Fixed
A fixed intrinsic size means that a layout node will be a fixed width or height. Fixed sizing is 
respected by all layout nodes during constraint calculations so, for example, if a layout node
has a fixed size of `500.0` then it will be `500.0` no matter what. This is useful but can often 
lead to bugs if misused, in fact most of the errors you encounter will mostly be caused by some fixed
node.

Fixed sizing is most prominently used for text and icons.

## Example layouts

## Comparison to CSS
- Justify between: There is no justify between, you may use nodes in between and set the flex to 1.
## License

Licensed under either of:

Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## TODO

- Add benchmarks