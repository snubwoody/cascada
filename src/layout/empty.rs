use crate::{
    BoxConstraints, BoxSizing, GlobalId, IntrinsicSize, Layout, LayoutIter, Position, Size,
};
use crate::constraints::impl_constraints;

/// An empty [`Layout`] with no child notes.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EmptyLayout {
    id: GlobalId,
    pub(crate) size: Size,
    position: Position,
    intrinsic_size: IntrinsicSize,
    constraints: BoxConstraints,
    errors: Vec<crate::LayoutError>,
    label: Option<String>,
}

impl EmptyLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_id(mut self, id: GlobalId) -> Self {
        self.id = id;
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    impl_constraints!();
}

impl Layout for EmptyLayout {
    fn label(&self) -> String {
        self.label.clone().unwrap_or("EmptyLayout".to_string())
    }

    fn solve_min_constraints(&mut self) -> (f32, f32) {
        if let BoxSizing::Fixed(width) = self.intrinsic_size.width {
            self.constraints.min_width = width;
        }

        if let BoxSizing::Fixed(height) = self.intrinsic_size.height {
            self.constraints.min_height = height;
        }

        (self.constraints.min_width, self.constraints.min_height)
    }

    // No children to solve for
    fn solve_max_constraints(&mut self, _: Size) {}

    fn position_children(&mut self) {}

    fn update_size(&mut self) {
        match self.intrinsic_size.width {
            BoxSizing::Flex(_) => {
                self.size.width = self.constraints.max_width.unwrap_or_default();
            }
            BoxSizing::Shrink => {
                self.size.width = self.constraints.min_width;
            }
            BoxSizing::Fixed(width) => {
                self.size.width = width;
            }
        }

        match self.intrinsic_size.height {
            BoxSizing::Flex(_) => {
                self.size.height = self.constraints.max_height;
            }
            BoxSizing::Shrink => {
                self.size.height = self.constraints.min_height;
            }
            BoxSizing::Fixed(height) => {
                self.size.height = height;
            }
        }
    }

    fn collect_errors(&mut self) -> Vec<crate::LayoutError> {
        self.errors.drain(..).collect::<Vec<_>>()
    }

    fn id(&self) -> GlobalId {
        self.id
    }

    fn constraints(&self) -> BoxConstraints {
        self.constraints
    }

    fn get_intrinsic_size(&self) -> IntrinsicSize {
        self.intrinsic_size
    }

    fn size(&self) -> Size {
        self.size
    }

    fn position(&self) -> Position {
        self.position
    }

    fn children(&self) -> &[Box<dyn Layout>] {
        &[]
    }

    fn set_max_width(&mut self, width: f32) {
        self.constraints.max_width = Some(width);
    }

    fn set_max_height(&mut self, height: f32) {
        self.constraints.max_height = height;
    }

    fn set_min_width(&mut self, width: f32) {
        self.constraints.min_width = width;
    }

    fn set_min_height(&mut self, height: f32) {
        self.constraints.min_height = height;
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_x(&mut self, x: f32) {
        self.position.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.position.y = y;
    }

    fn iter(&self) -> LayoutIter<'_> {
        LayoutIter { stack: vec![self] }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::solve_layout;

    #[test]
    fn test_flex_sizing() {
        let window = Size::new(800.0, 800.0);
        let mut root = EmptyLayout::new();

        root.intrinsic_size.width = BoxSizing::Flex(2);
        root.intrinsic_size.height = BoxSizing::Flex(2);

        solve_layout(&mut root, window);

        assert_eq!(root.size(), window);
    }

    #[test]
    fn test_fixed_sizing() {
        let window = Size::new(800.0, 800.0);
        let mut root = EmptyLayout::new();

        root.intrinsic_size.width = BoxSizing::Fixed(200.0);
        root.intrinsic_size.height = BoxSizing::Fixed(125.0);

        solve_layout(&mut root, window);

        assert_eq!(root.size(), Size::new(200.0, 125.0));
    }

    #[test]
    fn test_shrink_sizing() {
        let window = Size::new(800.0, 800.0);
        let mut root = EmptyLayout::new();

        solve_layout(&mut root, window);

        assert_eq!(root.size(), Size::default());
    }
}
