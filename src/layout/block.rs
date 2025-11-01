use crate::{
    AxisAlignment, BoxConstraints, BoxSizing, EmptyLayout, GlobalId, IntrinsicSize, Layout,
    LayoutError, LayoutIter, Padding, Position, Size,
};

/// A [`Layout`] that only has one child node.
///
/// # Example
/// ```
/// use cascada::{solve_layout, BlockLayout, EmptyLayout, HorizontalLayout, Padding, Size};
///
/// let child = HorizontalLayout::new()
///     .add_children([
///         EmptyLayout::new(),
///         EmptyLayout::new(),
///         EmptyLayout::new(),
///     ])
///     .spacing(12);
///
/// let mut block = BlockLayout::new(child)
///     .padding(Padding::all(20.0));
///
/// solve_layout(&mut block, Size::unit(200.0));
/// ```
#[derive(Debug)]
pub struct BlockLayout {
    id: GlobalId,
    pub(crate) size: Size,
    position: Position,
    padding: Padding,
    intrinsic_size: IntrinsicSize,
    constraints: BoxConstraints,
    main_axis_alignment: AxisAlignment,
    cross_axis_alignment: AxisAlignment,
    child: Box<dyn Layout>,
    errors: Vec<LayoutError>,
    label: Option<String>,
}

impl Default for BlockLayout {
    fn default() -> Self {
        Self {
            id: GlobalId::new(),
            size: Size::default(),
            padding: Padding::default(),
            position: Position::default(),
            intrinsic_size: IntrinsicSize::default(),
            constraints: BoxConstraints::default(),
            main_axis_alignment: AxisAlignment::default(),
            cross_axis_alignment: AxisAlignment::default(),
            errors: vec![],
            child: Box::new(EmptyLayout::default()),
            label: None,
        }
    }
}

impl BlockLayout {
    pub fn new<L: Layout + 'static>(child: L) -> Self {
        Self {
            child: Box::new(child),
            ..Default::default()
        }
    }

    pub fn child(&self) -> &dyn Layout {
        self.child.as_ref()
    }

    pub fn set_id(mut self, id: GlobalId) -> Self {
        self.id = id;
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Set the intrinsic size.
    pub fn intrinsic_size(mut self, intrinsic_size: IntrinsicSize) -> Self {
        self.intrinsic_size = intrinsic_size;
        self
    }

    /// Set the [`Padding`].
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Set the main axis alignment
    pub fn main_axis_alignment(mut self, main_axis_alignment: AxisAlignment) -> Self {
        self.main_axis_alignment = main_axis_alignment;
        self
    }

    /// Set the cross axis alignment.
    pub fn cross_axis_alignment(mut self, cross_axis_alignment: AxisAlignment) -> Self {
        self.cross_axis_alignment = cross_axis_alignment;
        self
    }

    fn align_main_axis_start(&mut self) {
        let mut x_pos = self.position.x;
        x_pos += self.padding.left;
        self.child.set_x(x_pos);
    }

    /// Align the children on the main axis in the center
    fn align_main_axis_center(&mut self) {
        let center_start = self.position.x + (self.size.width - self.child.size().width) / 2.0;
        self.child.set_x(center_start);
    }

    fn align_main_axis_end(&mut self) {
        let mut x_pos = self.position.x + self.size.width;
        x_pos -= self.padding.right;

        self.child.set_x(x_pos);
    }

    fn align_cross_axis_start(&mut self) {
        let y = self.position.y + self.padding.top;
        self.child.set_y(y);
    }

    fn align_cross_axis_center(&mut self) {
        let y_pos = (self.size.height - self.child.size().height) / 2.0 + self.position.y;
        self.child.set_y(y_pos);
    }

    fn align_cross_axis_end(&mut self) {
        self.child
            .set_y(self.position.y + self.size.height - self.padding.bottom);
    }
}

impl Layout for BlockLayout {
    fn label(&self) -> String {
        self.label.clone().unwrap_or("BlockLayout".to_string())
    }

    fn id(&self) -> GlobalId {
        self.id
    }

    fn size(&self) -> Size {
        self.size
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

    fn position(&self) -> Position {
        self.position
    }

    fn children(&self) -> &[Box<dyn Layout>] {
        std::slice::from_ref(&self.child)
    }

    fn constraints(&self) -> BoxConstraints {
        self.constraints
    }

    fn get_intrinsic_size(&self) -> IntrinsicSize {
        self.intrinsic_size
    }

    fn set_max_height(&mut self, height: f32) {
        self.constraints.max_height = height;
    }

    fn set_max_width(&mut self, width: f32) {
        self.constraints.max_width = Some(width);
    }

    fn set_min_height(&mut self, height: f32) {
        self.constraints.min_height = height;
    }

    fn set_min_width(&mut self, width: f32) {
        self.constraints.min_width = width;
    }

    fn collect_errors(&mut self) -> Vec<LayoutError> {
        self.errors
            .drain(..)
            .chain(self.child.collect_errors())
            .collect::<Vec<_>>()
    }

    fn iter(&self) -> LayoutIter<'_> {
        LayoutIter { stack: vec![self] }
    }

    fn solve_min_constraints(&mut self) -> (f32, f32) {
        let (min_width, min_height) = self.child.solve_min_constraints();

        // Set our min constraints to child + padding if intrinsic size
        // is not fixed.
        // If intrinsic size is fixed then set min constraints to fixed
        // width and/or height.
        match self.intrinsic_size.width {
            BoxSizing::Flex(_) | BoxSizing::Shrink => {
                self.constraints.min_width = self.padding.left + self.padding.right + min_width;
            }
            BoxSizing::Fixed(width) => self.constraints.min_width = width,
        }

        match self.intrinsic_size.height {
            BoxSizing::Flex(_) | BoxSizing::Shrink => {
                self.constraints.min_height = self.padding.top + self.padding.bottom + min_height;
            }
            BoxSizing::Fixed(height) => self.constraints.min_height = height,
        }

        (self.constraints.min_width, self.constraints.min_height)
    }

    fn solve_max_constraints(&mut self, space: Size) {
        let mut available_space = space;
        available_space.width -= self.padding.horizontal_sum();
        available_space.height -= self.padding.vertical_sum();

        // TODO: should layout set max constraints when shrink?
        match self.child.get_intrinsic_size().width {
            BoxSizing::Flex(_) => {
                self.child.set_max_width(available_space.width);
            }
            BoxSizing::Fixed(width) => {
                self.child.set_max_width(width);
            }
            BoxSizing::Shrink => {}
        }

        match self.child.get_intrinsic_size().height {
            BoxSizing::Flex(_) => {
                self.child.set_max_height(available_space.height);
            }
            BoxSizing::Fixed(height) => {
                self.child.set_max_height(height);
            }
            BoxSizing::Shrink => {}
        }

        self.child.solve_max_constraints(available_space);
    }

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

        self.child.update_size();
    }

    fn position_children(&mut self) {
        match self.main_axis_alignment {
            AxisAlignment::Start => self.align_main_axis_start(),
            AxisAlignment::Center => self.align_main_axis_center(),
            AxisAlignment::End => self.align_main_axis_end(),
        }

        match self.cross_axis_alignment {
            AxisAlignment::Start => self.align_cross_axis_start(),
            AxisAlignment::Center => self.align_cross_axis_center(),
            AxisAlignment::End => self.align_cross_axis_end(),
        }

        if self.child.position().x > self.position.x + self.size.width
            || self.child.position().y > self.position.y + self.size.height
        {
            self.errors.push(LayoutError::OutOfBounds {
                parent_id: self.id,
                child_id: self.child.id().to_owned(),
            });
        }
        self.child.position_children();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{EmptyLayout, solve_layout};

    #[test]
    fn flex_max_constraints() {
        let layout = EmptyLayout::new().intrinsic_size(IntrinsicSize::fill());

        let mut layout = BlockLayout::new(layout);
        layout.solve_max_constraints(Size::new(100.0, 200.0));
        assert_eq!(layout.child.constraints().max_width.unwrap(), 100.0);
        assert_eq!(layout.child.constraints().max_height, 200.0);
    }

    #[test]
    fn flex_max_constraints_with_padding() {
        let layout = EmptyLayout::new().intrinsic_size(IntrinsicSize::fill());

        let mut layout = BlockLayout::new(layout);
        layout.padding = Padding::new(10.0, 15.0, 20.0, 25.0);
        layout.solve_max_constraints(Size::new(100.0, 200.0));
        assert_eq!(layout.child.constraints().max_width.unwrap(), 100.0 - 25.0);
        assert_eq!(layout.child.constraints().max_height, 200.0 - 45.0);
    }

    #[test]
    fn fixed_min_constraints() {
        let child = EmptyLayout::default();
        let mut layout = BlockLayout::new(child);
        layout.intrinsic_size = IntrinsicSize::fixed(20.0, 500.0);
        layout.solve_min_constraints();

        assert_eq!(layout.constraints.min_width, 20.0);
        assert_eq!(layout.constraints.min_height, 500.0);
    }

    #[test]
    fn no_padding_in_fixed_min_constraints() {
        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(24.2, 24.0));

        let mut layout = BlockLayout::new(child);
        layout.intrinsic_size = IntrinsicSize::fixed(20.0, 500.0);
        layout.solve_min_constraints();
        layout.padding = Padding::all(24.0);

        assert_eq!(layout.constraints.min_width, 20.0);
        assert_eq!(layout.constraints.min_height, 500.0);
    }

    #[test]
    fn shrink_min_constraints() {
        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(20.0, 20.0));
        let mut layout = BlockLayout::new(child);
        layout.intrinsic_size = IntrinsicSize::shrink();
        layout.solve_min_constraints();

        assert_eq!(layout.constraints.min_width, 20.0);
        assert_eq!(layout.constraints.min_height, 20.0);
    }

    #[test]
    fn include_padding_shrink_min_constraints() {
        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(20.0, 20.0));
        let mut layout = BlockLayout::new(child);
        layout.intrinsic_size = IntrinsicSize::shrink();
        layout.padding = Padding::new(10.0, 15.0, 93.0, 53.0);
        layout.solve_min_constraints();

        assert_eq!(layout.constraints.min_width, 20.0 + 10.0 + 15.0);
        assert_eq!(layout.constraints.min_height, 20.0 + 93.0 + 53.0);
    }

    #[test]
    fn shrink_sizing() {
        let window = Size::new(800.0, 800.0);
        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(200.0, 200.0));

        let mut root = BlockLayout::new(child);
        root.padding = Padding::all(24.0);
        solve_layout(&mut root, window);

        let value = 24.0f32.mul_add(2.0, 200.0);
        assert_eq!(root.size(), Size::unit(value));
    }

    #[test]
    fn align_main_axis_start() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(child);
        root.padding = Padding::new(0.0, 24.0, 24.24, 24.2);
        root.align_main_axis_start();
        let pos = root.child.position();
        assert_eq!(pos.x, root.position().x);
        root.padding.left = 10.0;
        root.align_main_axis_start();
        assert_eq!(root.child.position().x, root.position().x + 10.0);
    }

    #[test]
    fn align_main_axis_end() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(child);
        root.size = Size::unit(200.0);
        root.align_main_axis_end();
        let pos = root.child.position();
        assert_eq!(pos.x, 200.0);
    }

    #[test]
    fn align_cross_axis_start() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(child);
        root.position.y = 50.0;
        root.align_cross_axis_start();
        let pos = root.child.position();
        assert_eq!(pos.y, 50.0);
    }

    #[test]
    fn align_cross_axis_start_uses_top_padding() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(child);
        root.position.y = 50.0;
        root.padding.top = 10.0;
        root.align_cross_axis_start();
        let pos = root.child.position();
        assert_eq!(pos.y, 60.0);
    }

    #[test]
    fn align_main_axis_end_uses_right_padding() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(child);
        root.padding = Padding::new(20.0, 50.0, 24.24, 24.2);
        root.size = Size::unit(200.0);
        root.align_main_axis_end();
        let pos = root.child.position();
        assert_eq!(pos.x, 200.0 - 50.0);
    }

    #[test]
    fn nested_shrink() {
        let window = Size::new(800.0, 800.0);

        let inner_child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(175.0, 15.0));

        let mut child = BlockLayout::new(inner_child);
        child.padding = Padding::all(24.0);

        let mut root = BlockLayout::new(child);

        solve_layout(&mut root, window);

        let inner_size = Size::new(175.0, 15.0);
        let child_size = inner_size + 24.0 * 2.0;

        assert_eq!(root.size(), child_size);
        assert_eq!(root.child.size(), child_size);
        assert_eq!(root.child.children()[0].size(), inner_size);
    }

    #[test]
    fn grow() {
        let window = Size::new(800.0, 800.0);
        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fill());

        let padding = Padding::all(24.0);
        let mut root = BlockLayout::new(child);
        root.intrinsic_size.width = BoxSizing::Flex(1);
        root.intrinsic_size.height = BoxSizing::Flex(1);
        root.padding = padding;

        solve_layout(&mut root, window);

        let mut child_size = window;
        child_size.width -= padding.horizontal_sum();
        child_size.height -= padding.vertical_sum();
        assert_eq!(root.size(), window);
        assert_eq!(root.child.size(), child_size);
    }

    #[test]
    fn inner_grow() {
        let window = Size::new(800.0, 800.0);
        let inner_child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fill());

        let mut child = BlockLayout::new(inner_child);
        child.intrinsic_size = IntrinsicSize::fill();

        let mut root = BlockLayout::new(child);
        root.intrinsic_size = IntrinsicSize::fill();
        root.padding = Padding::all(24.0);

        solve_layout(&mut root, window);

        let mut child_size = window;
        child_size.width -= root.padding.horizontal_sum();
        child_size.height -= root.padding.vertical_sum();
        assert_eq!(root.size(), window);
        assert_eq!(root.child.size(), child_size);
        assert_eq!(root.child.size(), root.child.children()[0].size());
    }
}
