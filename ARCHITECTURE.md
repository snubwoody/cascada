
The layout tree is made up of `Layout` nodes.

The core concepts are [intrinsic size](https://en.wikipedia.org/wiki/Intrinsic_dimension)
and constraints. The intrinsic size is the preferred/requested size of a 
layout node, so for example a node could specify that it wants be as large as possible.

Constraints define the minimum and maximum width and height that a layout node can be.

**Properties of layout nodes:**

Not all layout nodes have all properties.

- Padding: The spacing between the edges of the node and it's content.
- Spacing: The spacing between child nodes.
- Main axis alignment: Specifies how to align child nodes on the main axis.
- Cross axis alignment: Specifies how to align child nodes on the cross axis.
- Constraints: The minimum and maximum size of a node.
- Intrinsic size: The preferred size of a node.

**Layout types:**

- `EmptyLayout`: A simple layout node with no child nodes.
- `BlockLayout`: A layout node with one child node.
- `HorizontalLayout`: A layout node with multiple child nodes flowing along the x-axis.
- `VerticalLayout`: A layout node with multiple child nodes flowing along the y-axis.

## Passes
The layout system performs two passes to determine node sizes.

The first pass is for the minimum constraints, starting from the bottom the layout
nodes will return their min constraints to their parents which use these constraints
to calculate their own and in turn pass the constraints back up to the parents.

The second pass is for maximum constraints, these start at the root element, each node
will tell it's child nodes how much available space there is and the constraints will
pass down the tree.

After these passes each layout node can now adjust it's size, nodes with `Shrink` sizing
will use their min constraints, nodes with `Flex` sizing will use their max 
constraints and nodes with `Fixed` sizing will use the specified width or height.

## Axes
Every layout node has two axes:

- Main axis: The axis along which content flows.
- Cross axis: The axis perpendicular to the main axis.

For most layouts the main axis is the x-axis and the cross axis is y-axis. The only 
exception currently is the 
[`VerticalLayout`](https://docs.rs/cascada/latest/cascada/vertical/struct.VerticalLayout.html)
whose main axis is the y-axis and cross axis is the x-axis.

![axis-alignment](./art/axis-alignment.svg)

