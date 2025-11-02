//! Cascada is a lightweight, high-performance layout engine for UI frameworks.
//!
//! It is designed for developers building UI libraries, such as GUIs or TUI's, who
//! want a fast, predictable layout system without the complexity of
//! implementing their own.
//!
//! # Usage
//! The core of this library is the [`Layout`] trait, which is implemented for different
//! use cases. There are currently four types of layout nodes:
//!
//! - [`EmptyLayout`]
//! - [`BlockLayout`]
//! - [`HorizontalLayout`]
//! - [`VerticalLayout`]
//!
//! Create a root layout node and pass it into the [`solve_layout`] function with the total
//! available space.
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
//! To get the size and position of the layout nodes you can iterate over the tree.
//!
//! ```
//! use cascada::{HorizontalLayout, EmptyLayout, solve_layout, IntrinsicSize, Size, Layout};
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
//! for node in layout.iter(){
//!     println!("Size: {:?}",node.size());
//!     println!("Position: {:?}",node.position());
//! }
//! ```
//!
//! Or you could use ids to get specific nodes from the tree.
//!
//! ```
//! use cascada::{HorizontalLayout, EmptyLayout, solve_layout, IntrinsicSize, Size, Layout, GlobalId};
//! let id = GlobalId::new();
//!
//! let child = EmptyLayout::new()
//!     .set_id(id)
//!     .intrinsic_size(IntrinsicSize::fixed(20.0,50.0));
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
//!
//! let child = layout.get(id).unwrap();
//! assert_eq!(child.size().width,20.0);
//! ```
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::suspicious_operation_groupings)]
#![warn(clippy::imprecise_flops)]
mod constraints;
#[cfg(feature = "debug-tools")]
pub mod debug;
mod error;
mod layout;
mod position;
mod size;

pub use constraints::*;
pub use error::LayoutError;
pub use layout::*;
pub use position::Bounds;
pub use position::Position;
pub use size::Size;
use std::fmt::Debug;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A global unique identifier
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Debug, Ord, Hash)]
pub struct GlobalId(u32);

impl GlobalId {
    pub fn new() -> Self {
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for GlobalId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for GlobalId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
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

/// The space between the edges of a [`Layout`] node and its content.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
pub struct Padding {
    /// The left padding.
    pub left: f32,
    /// The right padding.
    pub right: f32,
    /// The top padding.
    pub top: f32,
    /// The bottom padding.
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
