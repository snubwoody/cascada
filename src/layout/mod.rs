use crate::{Bounds, BoxConstraints, GlobalId, IntrinsicSize, LayoutError, Position, Size};
use std::fmt::Debug;

pub mod block;
pub mod empty;
pub mod horizontal;
pub mod vertical;

pub use block::BlockLayout;
pub use empty::EmptyLayout;
pub use horizontal::HorizontalLayout;
pub use vertical::VerticalLayout;

/// Solve the final size and position of all the layout nodes. The
/// `window_size` is the maximum available space for the root node.
///
/// This functions
/// returns any layout errors such as overflow or out of bounds.
///
/// # Example
///
/// ```
/// use cascada::{solve_layout, BlockLayout, EmptyLayout, IntrinsicSize, Padding, Size};
///
/// let child = EmptyLayout::new()
///     .intrinsic_size(IntrinsicSize::fixed(50.0,50.0));
/// let mut block = BlockLayout::new(child)
///     .padding(Padding::all(10.0));
///
/// let errors = solve_layout(&mut block,Size::unit(500.0));
/// assert!(errors.is_empty());
/// ```
pub fn solve_layout(root: &mut dyn Layout, window_size: Size) -> Vec<LayoutError> {
    if root.constraints().max_width.is_none() {
        root.set_max_width(window_size.width);
    }
    root.set_max_height(window_size.height);

    // It's important that the min constraints are solved before the max constraints
    // because the min constraints are used in calculating max constraints.
    let _ = root.solve_min_constraints();
    root.solve_max_constraints(window_size);
    root.update_size();
    root.position_children();

    root.collect_errors()
}

/// A layout node.
pub trait Layout: Debug + private::Sealed {
    fn label(&self) -> String;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn root_max_width() {
        let mut layout = EmptyLayout::new()
            .max_width(20.0)
            .intrinsic_size(IntrinsicSize::fill());

        solve_layout(&mut layout, Size::unit(200.0));
        assert_eq!(layout.size().width, 20.0);
    }
}
