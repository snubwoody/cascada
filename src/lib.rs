//! Cascada is a lightweight, high-performance layout engine for UI frameworks.
//!
//! # Example
//!
//! ```
//! use cascada::{HorizontalLayout, EmptyLayout, solve_layout, IntrinsicSize, Size, Layout};
//!
//! // Add three equally sized nodes.
//! let child = EmptyLayout::new()
//!     .intrinsic_size(IntrinsicSize::fill());
//!
//! let mut layout = HorizontalLayout::new()
//!     .intrinsic_size(IntrinsicSize::fill())
//!     .add_children([
//!         child.clone(),
//!         child.clone(),
//!         child,
//!     ]);
//!
//! solve_layout(&mut layout,Size::unit(3000.0));
//!
//! let size = &layout.children()[0].size();
//! assert_eq!(size.width,1000.0);
//! ```
//!
//! ## Layout engine
//! `cascada` is a two pass layout engine that uses `contraints` and [`IntrinsicSize`] to solve the layout 
//! tree. Minimum constraints flow up and maximum constraints flow down.
//!
//! The maximum size starts from the top, as it goes down the widget tree the nodes are given the
//! maximum size they can take up, and similarly give their child nodes the maximum they can take
//! up.
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

/// Solve the final size and position of all the layout nodes.
pub fn solve_layout(root: &mut dyn Layout, window_size: Size) -> Vec<LayoutError> {
    root.set_max_width(window_size.width);
    root.set_max_height(window_size.height);

    // It's important that the min constraints are solved before the max constraints
    // because the min constraints are used in calculating max constraints.
    let _ = root.solve_min_constraints();
    root.solve_max_constraints(window_size);
    root.update_size();
    root.position_children();

    // FIXME
    // root.collect_errors();
    vec![]
}

/// A layout node.
///
/// ## Details
///
/// Each layout node has minimum and maximum constraints, these describe the bounds
/// of the layout node i.e. the max and min space it's allowed to take up.
///
/// ### Axes
/// Each node has two axes: the main axis and the cross axis. The main axis is the
/// axis along which content flows and the cross axis is the axis perpendicular
/// to the cross axis. For most nodes the main axis is the x-axis while the
/// cross axis is the y-axis.
pub trait Layout: Debug + private::Sealed {
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
    fn get_intrinsic_size(&self) -> IntrinsicSize;

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

    /// Iterate over the layout tree.
    fn iter(&self) -> LayoutIter<'_>;

    /// Get a [`Layout`] by it's `id`.
    fn get(&self, id: GlobalId) -> Option<&dyn Layout> {
        self.iter().find(|&layout| layout.id() == id)
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for super::EmptyLayout {}
    impl Sealed for super::BlockLayout {}
    impl Sealed for super::HorizontalLayout {}
    impl Sealed for super::VerticalLayout {}
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

/// Describes how a [`Layout`] should align its children.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum AxisAlignment {
    /// Place content at the start.
    #[default]
    Start,
    /// Place content in the center.
    Center,
    /// Place content at the end.
    End,
}

/// Describes the maximum and minimum size of a [`Layout`].
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BoxConstraints {
    pub max_width: f32,
    pub max_height: f32,
    pub min_height: f32,
    pub min_width: f32,
}

impl BoxConstraints {
    /// Create new [`BoxConstraints`].
    pub fn new() -> Self {
        Self::default()
    }
}

/// This is the preferred size of a [`Layout`] node. 
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct IntrinsicSize {
    pub width: BoxSizing,
    pub height: BoxSizing,
}

impl IntrinsicSize {
    /// Create an intrinsic size with a flex factor of `1`.
    ///
    /// # Example
    /// ```
    /// use cascada::{BoxSizing,IntrinsicSize};
    ///
    /// let instrinsic_size = IntrinsicSize::fill();
    ///
    /// assert_eq!(instrinsic_size.width,BoxSizing::Flex(1));
    /// assert_eq!(instrinsic_size.height,BoxSizing::Flex(1));
    /// ```
    pub const fn fill() -> Self {
        Self {
            width: BoxSizing::Flex(1),
            height: BoxSizing::Flex(1),
        }
    }

    /// Creates an intrinsic size with a flex factor.
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::{BoxSizing, IntrinsicSize};
    ///
    /// let intrinsic_size = IntrinsicSize::flex(8);
    ///
    /// assert_eq!(intrinsic_size.width,BoxSizing::Flex(8));
    /// assert_eq!(intrinsic_size.height,BoxSizing::Flex(8));
    /// ```
    pub const fn flex(factor: u8) -> Self {
        Self {
            width: BoxSizing::Flex(factor),
            height: BoxSizing::Flex(factor),
        }
    }

    /// Creates an [`IntrinsicSize`] that shrinks to fit its contents.
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::{BoxSizing, IntrinsicSize};
    ///
    /// let intrinsic_size = IntrinsicSize::shrink();
    ///
    /// assert_eq!(intrinsic_size.width, BoxSizing::Shrink);
    /// assert_eq!(intrinsic_size.height, BoxSizing::Shrink);
    /// ```
    pub const fn shrink() -> Self {
        Self {
            width: BoxSizing::Shrink,
            height: BoxSizing::Shrink,
        }
    }

    /// Creates an [`IntrinsicSize`] with a fixed size.
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
    pub const fn fixed(width: f32, height: f32) -> Self {
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

/// The space between the edges of a [`Layout`] node and its content.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
    /// Creates a new [`Padding`].
    ///
    /// # Panics
    /// Panics if sides are negative.
    pub const fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
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

    /// Create padding with symmetric vertical and horizontal sides.
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::Padding;
    ///
    /// let padding = Padding::symmetric(10.0,20.0);
    ///
    /// assert_eq!(padding.top,10.0);
    /// assert_eq!(padding.left,20.0);
    /// assert_eq!(padding.left,padding.right);
    /// assert_eq!(padding.bottom,padding.top);
    /// ```
    pub const fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self::new(horizontal, horizontal, vertical, vertical)
    }

    /// Create a [`Padding`] with equal sides.
    ///
    /// # Example
    /// ```
    /// use cascada::Padding;
    ///
    /// let padding = Padding::all(20.0);
    ///
    /// assert_eq!(padding.left,20.0);
    /// assert_eq!(padding.left,padding.right);
    /// assert_eq!(padding.bottom,padding.top);
    /// ```
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

    /// The sum of all the padding sides.
    ///
    /// # Example
    /// ```
    /// use cascada::Padding;
    ///
    /// let padding = Padding::all(24.0);
    ///
    /// assert_eq!(padding.sum(),24.0 * 4.0);
    /// ```
    pub const fn sum(&self) -> f32 {
        self.horizontal_sum() + self.vertical_sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn padding_no_negative() {
        Padding::new(0.0, 0.0, 0.0, -35.0);
    }
}
