//!
//!
//! ## Layout engine
//! `cascada` is a two pass layout engine that uses `contraints` to solve the layout tree. Minimum
//! constraints flow up and maximum constraints flow down.
//!
//! The maximum size starts from the top, as it goes down the widget tree the nodes are given the
//! maximum size they can take up, and similarly give their child nodes the maximum they can take
//! up.
//!
//! ## Constraints
//! Constraints define the minimum and maximum bounds of a layout node, which are the minimum
//! and maximum size it can take up. Every layout node has minimum and maximum constraints, i.e minimum and maximum width and height. Maximum constraints are set by the parents and passed
//! down the tree, while minimum constraints are set the node itself and passed up the tree. Hence why this is a two pass layout engine, the minimum constraints start at the bottom going up, while the maximum constraints start at the top going down.
//!
//! [TODO: diagram](#)
//!
//! ## Padding
//! Padding is the space between the edges of a layout node and its content, the padding struct
//! has 4 sides: `left`, `right`, `top` and `bottom`.
//!
//! ## Axes
//! Each node has two axes: the main axis and the cross axis. The main axis is the axis along which content flows and the cross axis is the axis perpendicular to the cross axis.
//!
//! ### Alignment
//! There are three `AxisAlignment` variants that specify how a node should align its children i.e.
//!
//! - `AxisAlignment::Start`: Align content at the start of the axis.
//! - `AxisAlignment::Center`: Align content in the center of the axis.
//! - `AxisAlignement::End`: Align content at the end of the axis.
//!
//! ```text
//! |----------------------------|
//! |                            |
//! |                            |
//! |                          | |
//! |                          | |
//! |____________________________|
//! ```
//!
//! TODO: Add figma diagrams
//!
//! ## Layouts
//!
//! ### Horizontal layout
//! This is a layout node that arranges it's content along the x-axis.
//!
//! ### Vertical layout
//!
//! ### Block layout
//!
//! ### Empty layout
//! A layout node with no children. The distinction between no children, one child and multiple children
//! is important, which is why they are separate. This is usually used for graphical elements such as
//! text, images, icons and so on. Due to the fact that they have no children, internally, empty layouts
//! get to skip a lot of the calculations.
//!
//! ## Intrinsic size
//! Intrinsic size is the size that a layout node requests to be, for example, filling the screen.
//!
//! For example to have two equally sized nodes in a horizontal node you would give them an intrinsic
//! width of `Flex`.
//!
//! ### Shrink
//! Shrink sizing means a layout node wants to be as small as possible, for nodes with child nodes this
//! mean that they will fit their children. This is similar to
//! [`fit-content`](https://developer.mozilla.org/en-US/docs/Web/CSS/fit-content) in CSS.
//!
//! ### Fixed
//! A fixed intrinsic size means that a layout node will be a fixed width or height. Fixed sizing is
//! respected by all layout nodes during constraint calculations so, for example, if a layout node
//! has a fixed size of `500.0` then it will be `500.0` no matter what. This is useful but can often
//! lead to bugs if misused, in fact most of the errors you encounter will mostly be caused by some fixed
//! node.
//!
//! Fixed sizing is most prominently used for text and icons.
//!
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::suspicious_operation_groupings)]
#![warn(clippy::imprecise_flops)]
pub mod block;
pub mod empty;
mod error;
pub mod horizontal;
mod position;
mod size;
pub mod vertical;

use agape_core::GlobalId;
pub use block::BlockLayout;
pub use empty::EmptyLayout;
pub use error::LayoutError;
pub use horizontal::HorizontalLayout;
pub use position::Bounds;
pub use position::Position;
pub use size::Size;
use std::fmt::Debug;
pub use vertical::VerticalLayout;

/// Calculates the layout of all the layout nodes
pub fn solve_layout(root: &mut dyn Layout, window_size: Size) -> Vec<LayoutError> {
    root.set_max_width(window_size.width);
    root.set_max_height(window_size.height);

    // It's important that the min constraints are solved before the max constraints
    // because the min constraints are used in calculating max constraints
    let _ = root.solve_min_constraints();
    root.solve_max_constraints(window_size);
    root.update_size();
    root.position_children();

    // TODO add a push error function that checks for equality so that we don't have duplicate errors
    // or maybe just clear the error stack every frame
    // root.collect_errors()
    vec![]
}

// TODO: add anchor layout and grid layout
pub trait Layout: Debug + Send + Sync {
    /// Solve the minimum constraints of each [`Layout`] node recursively
    fn solve_min_constraints(&mut self) -> (f32, f32);

    /// Solve the max constraints for the children and pass them down the tree
    fn solve_max_constraints(&mut self, space: Size);

    /// Position the layout nodes after size calculations.
    fn position_children(&mut self);

    /// Update the size of every [`LayoutNode`] based on it's size and constraints.
    fn update_size(&mut self);

    /// Collect all the errors from the node tree.
    fn collect_errors(&mut self) -> Vec<LayoutError>;

    /// Get the `id` of the [`Layout`]
    fn id(&self) -> GlobalId;

    /// Get the [`BoxConstraints`] of the [`Layout`]
    fn constraints(&self) -> BoxConstraints;

    /// Get the [`IntrinsicSize`] of the [`Layout`]
    fn intrinsic_size(&self) -> IntrinsicSize;

    /// Get the `Size` of the [`Layout`]
    fn size(&self) -> Size;

    /// Get the `Position` of the [`Layout`]
    fn position(&self) -> Position;

    /// Get the `Bounds` of the [`Layout`]
    fn bounds(&self) -> Bounds {
        Bounds::new(self.position(), self.size())
    }

    fn children(&self) -> &[Box<dyn Layout>];

    fn set_max_width(&mut self, width: f32);
    fn set_max_height(&mut self, height: f32);
    fn set_min_width(&mut self, width: f32);
    fn set_min_height(&mut self, height: f32);

    fn set_position(&mut self, position: Position) {
        self.set_x(position.x);
        self.set_y(position.y);
    }

    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);

    fn iter(&self) -> LayoutIter<'_>;

    /// Get a [`Layout`] by it's `id`.
    fn get(&self, id: GlobalId) -> Option<&dyn Layout> {
        self.iter().find(|&layout| layout.id() == id)
    }
}

/// An `Iterator` over the layout tree.
pub struct LayoutIter<'a> {
    stack: Vec<&'a dyn Layout>,
}

impl<'a> Iterator for LayoutIter<'a> {
    type Item = &'a dyn Layout;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(layout) = self.stack.pop() {
            let children = layout.children();
            let m = children.iter().map(|child| child.as_ref());
            self.stack.extend(m.rev());
            return Some(layout);
        }

        None
    }
}

/// Describes the size a [`Layout`] will try to be.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub enum BoxSizing {
    /// The [`Layout`] will be a fixed size regardless of any other conditions, this can
    /// cause overflow if not used wisely.
    Fixed(f32),
    /// Tries to be as small as possible
    #[default]
    Shrink,
    /// Tries to be as big as possible, the behaviour of the flex factor is
    /// dependent on the type of layout.
    Flex(u8),
}

/// Describes how a [`Layout`] should arrange its children
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum AxisAlignment {
    #[default]
    Start,
    Center,
    End,
}

/// Describes the maximum and minimum size of a [`Layout`]
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BoxConstraints {
    pub max_width: f32,
    pub max_height: f32,
    pub min_height: f32,
    pub min_width: f32,
}

impl BoxConstraints {
    /// Create new [`BoxConstraints`]
    pub fn new() -> Self {
        Self::default()
    }
}

/// This is the size that a [`Layout`] will try to be, the actual final size is
/// dependent on the space available.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct IntrinsicSize {
    pub width: BoxSizing,
    pub height: BoxSizing,
}

impl IntrinsicSize {
    pub fn fill() -> Self {
        Self {
            width: BoxSizing::Flex(1),
            height: BoxSizing::Flex(1),
        }
    }

    pub fn flex(factor: u8) -> Self {
        Self {
            width: BoxSizing::Flex(factor),
            height: BoxSizing::Flex(factor),
        }
    }

    pub fn shrink() -> Self {
        Self {
            width: BoxSizing::Shrink,
            height: BoxSizing::Shrink,
        }
    }

    /// Create a new fixed intrinsic size.
    ///
    /// # Example
    /// ```
    /// use cascada::{IntrinsicSize,BoxSizing};
    ///
    /// let intrinsic_size = IntrinsicSize::fixed(100.0,50.0);
    ///
    /// assert_eq!(intrinsic_size.width,BoxSizing::Fixed(100.0));
    /// assert_eq!(intrinsic_size.height,BoxSizing::Fixed(50.0));
    /// ```
    pub fn fixed(width: f32, height: f32) -> Self {
        Self {
            width: BoxSizing::Fixed(width),
            height: BoxSizing::Fixed(height),
        }
    }
}

impl From<Size> for IntrinsicSize {
    fn from(size: Size) -> Self {
        IntrinsicSize {
            width: BoxSizing::Fixed(size.width),
            height: BoxSizing::Fixed(size.height),
        }
    }
}

/// The spacing between the edges of a [`Layout`] node and its content.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
    /// Create a new [`Padding`].
    ///
    /// # Panics
    /// Panics if sides are negative.
    pub const fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        // TODO: test this
        assert!(
            left >= 0.0 && right >= 0.0 && top >= 0.0 && bottom >= 0.0,
            "Padding sides must be positive."
        );
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub const fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self::new(horizontal, horizontal, vertical, vertical)
    }

    /// Create a [`Padding`] with all sides equal.
    pub const fn all(padding: f32) -> Self {
        Self::new(padding, padding, padding, padding)
    }

    /// The sum of the top and bottom padding.
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::Padding;
    ///
    /// let padding = Padding::symmetric(20.0,10.0);
    ///
    /// assert_eq!(padding.vertical_sum(),40.0);
    /// ```
    pub const fn vertical_sum(&self) -> f32 {
        self.bottom + self.top
    }

    /// The sum of the left and right padding.
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::Padding;
    ///
    /// let padding = Padding::symmetric(20.0,10.0);
    ///
    /// assert_eq!(padding.horizontal_sum(),20.0);
    /// ```
    pub const fn horizontal_sum(&self) -> f32 {
        self.left + self.right
    }

    pub const fn sum(&self) -> f32 {
        self.horizontal_sum() + self.vertical_sum()
    }
}
