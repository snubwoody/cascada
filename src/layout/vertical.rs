use crate::{
    AxisAlignment, BoxConstraints, BoxSizing, GlobalId, IntrinsicSize, Layout, LayoutError,
    LayoutIter, Padding, Position, Size, error::OverflowAxis,
};
use crate::constraints::impl_constraints;

/// A [`Layout`] node that arranges it's children vertically.
///
/// # Example
/// ```
/// use cascada::{solve_layout, AxisAlignment, EmptyLayout, IntrinsicSize, Padding, Size, VerticalLayout};
///
/// let child = EmptyLayout::new()
///     .intrinsic_size(IntrinsicSize::fixed(12.0,50.0));
///
///
/// let mut layout = VerticalLayout::new()
///     .spacing(12)
///     .padding(Padding::all(24.0))
///     .add_children([child.clone(),child])
///     .main_axis_alignment(AxisAlignment::Center);
///
/// solve_layout(&mut layout, Size::unit(500.0));
/// ```
#[derive(Default, Debug)]
pub struct VerticalLayout {
    id: GlobalId,
    size: Size,
    position: Position,
    spacing: u32,
    padding: Padding,
    // TODO: maybe scrolling should be handled in
    // the UI layer instead
    scroll_offset: f32,
    intrinsic_size: IntrinsicSize,
    children: Vec<Box<dyn Layout>>,
    /// The main axis is the `y-axis`
    main_axis_alignment: AxisAlignment,
    /// The cross axis is the `x-axis`
    cross_axis_alignment: AxisAlignment,
    constraints: BoxConstraints,
    label: Option<String>,
    errors: Vec<LayoutError>,
}

impl VerticalLayout {
    /// Creates a new [`VerticalLayout`].
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

    /// Appends a [`Layout`] node to the list of children.
    ///
    /// # Example
    /// ```
    /// use cascada::{EmptyLayout,VerticalLayout};
    ///
    /// VerticalLayout::new()
    ///     .add_child(EmptyLayout::default())
    ///     .add_child(VerticalLayout::default());
    /// ```
    pub fn add_child(mut self, child: impl Layout + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    /// Add multiple child nodes to the list of children.
    ///
    /// # Example
    /// ```
    /// use cascada::{VerticalLayout,EmptyLayout};
    ///
    /// VerticalLayout::new()
    ///     .add_children([
    ///         EmptyLayout::new(),
    ///         EmptyLayout::new(),
    ///         EmptyLayout::new(),
    ///     ]);
    /// ```
    pub fn add_children<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item: Layout + 'static>,
    {
        for child in children {
            self.children.push(Box::new(child));
        }
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

    /// Set this layout's [`Padding`].
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Set this layout's spacing.
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Returns `true` if a [`VerticalLayout`]'s children are overflowing it's main-axis
    /// (y-axis).
    pub fn main_axis_overflow(&self) -> bool {
        self.errors
            .contains(&LayoutError::overflow(self.id, OverflowAxis::MainAxis))
    }

    /// Returns `true` if a [`VerticalLayout`]'s children are overflowing it's cross-axis
    /// (x-axis).
    pub fn cross_axis_overflow(&self) -> bool {
        self.errors
            .contains(&LayoutError::overflow(self.id, OverflowAxis::CrossAxis))
    }

    fn fixed_size_sum(&self) -> Size {
        let mut sum = Size::default();

        for child in self.children.iter() {
            if let BoxSizing::Fixed(width) = child.get_intrinsic_size().width {
                sum.width = sum.width.max(width);
            }

            match child.get_intrinsic_size().height {
                BoxSizing::Fixed(height) => {
                    sum.height += height;
                }
                BoxSizing::Shrink => {
                    sum.height += child.constraints().min_height;
                }
                _ => {}
            }
        }

        sum
    }

    pub(crate) fn scroll(&mut self, offset: f32) {
        self.scroll_offset += offset;
    }

    /// Align the children on the main axis at the start
    fn align_main_axis_start(&mut self) {
        let mut y = self.position.y;
        y += self.padding.top;

        for child in &mut self.children {
            child.set_y(y);
            y += child.size().height + self.spacing as f32;
        }
    }

    /// Align the children on the main axis in the center
    fn align_main_axis_center(&mut self) {
        let mut height_sum = self
            .children
            .iter()
            .map(|child| child.size().height)
            .sum::<f32>();

        // FIXME: panics with 0 children
        height_sum += (self.spacing * (self.children.len() as u32 - 1)) as f32;
        let mut center_start = self.position.y + (self.size.height - height_sum) / 2.0;

        for child in &mut self.children {
            child.set_y(center_start);
            center_start += child.size().height + self.spacing as f32;
        }
    }

    fn align_main_axis_end(&mut self) {
        let mut y = self.position.y + self.size.height;
        y -= self.padding.right;

        for child in self.children.iter_mut().rev() {
            child.set_y(y);
            y -= child.size().height - self.spacing as f32;
        }
    }

    fn align_cross_axis_start(&mut self) {
        let x = self.position.x + self.padding.top;
        for child in &mut self.children {
            child.set_x(x);
        }
    }

    fn align_cross_axis_center(&mut self) {
        for child in &mut self.children {
            let x_pos = (self.size.width - child.size().width) / 2.0 + self.position.x;
            child.set_x(x_pos);
        }
    }

    fn align_cross_axis_end(&mut self) {
        for child in &mut self.children {
            child.set_x(self.position.x + self.size.width - self.padding.right);
        }
    }

    fn compute_children_min_size(&mut self) -> Size {
        let mut sum = Size::default();
        sum.width += self.padding.horizontal_sum();
        sum.height += self.padding.vertical_sum();
        if self.children.is_empty() {
            return sum;
        }

        let space_between = (self.children.len() - 1) as f32 * self.spacing as f32;
        sum.height += space_between;
        let mut max_width = 0.0f32;
        for child in self.children.iter_mut() {
            let (min_width, min_height) = child.solve_min_constraints();
            sum.height += min_height;
            max_width = max_width.max(min_width);
        }
        sum.width += max_width;
        sum
    }

    impl_constraints!();
}

impl Layout for VerticalLayout {
    fn label(&self) -> String {
        self.label.clone().unwrap_or("VerticalLayout".to_string())
    }

    fn id(&self) -> GlobalId {
        self.id
    }

    fn position(&self) -> Position {
        self.position
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

    fn size(&self) -> Size {
        self.size
    }

    fn children(&self) -> &[Box<dyn Layout>] {
        self.children.as_slice()
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
            .chain(
                self.children
                    .iter_mut()
                    .flat_map(|child| child.collect_errors())
            )
            .collect::<Vec<_>>()
    }

    fn iter(&self) -> LayoutIter<'_> {
        LayoutIter { stack: vec![self] }
    }

    fn solve_min_constraints(&mut self) -> (f32, f32) {
        let child_constraint_sum = self.compute_children_min_size();

        match self.intrinsic_size.width {
            BoxSizing::Fixed(width) => {
                self.constraints.min_width = width;
            }
            BoxSizing::Flex(_) | BoxSizing::Shrink => {
                self.constraints.min_width = child_constraint_sum.width;
            }
        }

        match self.intrinsic_size.height {
            BoxSizing::Fixed(height) => {
                self.constraints.min_height = height;
            }
            BoxSizing::Flex(_) | BoxSizing::Shrink => {
                self.constraints.min_height = child_constraint_sum.height;
            }
        }

        (self.constraints.min_width, self.constraints.min_height)
    }

    fn solve_max_constraints(&mut self, _space: Size) {
        // Sum up all the flex factors
        let flex_total: u8 = self
            .children
            .iter()
            .filter_map(|child| {
                if let BoxSizing::Flex(factor) = child.get_intrinsic_size().height {
                    Some(factor)
                } else {
                    None
                }
            })
            .sum();

        let mut available_height;
        match self.intrinsic_size.height {
            BoxSizing::Shrink => {
                available_height = self.constraints.min_height;
                available_height -= self.fixed_size_sum().height;
            }
            BoxSizing::Fixed(_) | BoxSizing::Flex(_) => {
                available_height = self.constraints.max_height;
                available_height -= self.padding.horizontal_sum();
                available_height -= self.fixed_size_sum().height;
            }
        }

        let mut available_width;
        match self.intrinsic_size.width {
            BoxSizing::Shrink => available_width = self.constraints.min_width,
            BoxSizing::Fixed(_) | BoxSizing::Flex(_) => {
                available_width = self.constraints.max_width.unwrap_or_default();
                available_width -= self.padding.horizontal_sum();
            }
        }

        if !self.children.is_empty() {
            // Add the spacing between layouts
            for _ in 0..self.children.len() - 1 {
                available_height -= self.spacing as f32;
            }
        }

        for child in self.children.iter_mut() {
            match child.get_intrinsic_size().width {
                BoxSizing::Flex(_) => {
                    child.set_max_width(available_width);
                }
                BoxSizing::Shrink => {
                    child.set_max_width(child.constraints().min_width);
                }
                BoxSizing::Fixed(width) => {
                    child.set_max_width(width);
                }
            }

            match child.get_intrinsic_size().height {
                BoxSizing::Flex(factor) => {
                    let grow_factor = factor as f32 / flex_total as f32;
                    child.set_max_height(grow_factor * available_height);
                }
                BoxSizing::Fixed(height) => {
                    child.set_max_height(height);
                }
                BoxSizing::Shrink => {}
            }

            child.solve_max_constraints(Size::default());
        }
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

        for child in &mut self.children {
            child.update_size();
        }

        let width_sum: f32 = self.children.iter().map(|child| child.size().width).sum();
        let mut height_sum = self.padding.vertical_sum();
        for (i, child) in self.children.iter().enumerate() {
            height_sum += child.size().height;
            if i != self.children.len() - 1 {
                height_sum += self.spacing as f32;
            }
        }

        let main_axis_error = LayoutError::overflow(self.id, OverflowAxis::MainAxis);
        let cross_axis_error = LayoutError::overflow(self.id, OverflowAxis::CrossAxis);

        // Prevent duplicate errors
        if !self.errors.contains(&cross_axis_error) && width_sum > self.size.width {
            self.errors.push(cross_axis_error);
        }

        if !self.errors.contains(&main_axis_error) && height_sum > self.size.height {
            self.errors.push(main_axis_error);
        }
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

        for child in &mut self.children {
            let y = child.position().y;
            child.set_y(y + self.scroll_offset);

            if child.position().y > self.position.y + self.size.height {
                self.errors.push(LayoutError::OutOfBounds {
                    parent_id: self.id,
                    child_id: child.id().to_owned(),
                });
            }
            child.position_children();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{BlockLayout, EmptyLayout, Padding, solve_layout};

    #[test]
    fn calculate_min_width() {
        let widths: [f32; 5] = [500.0, 200.0, 10.2, 20.2, 45.0];
        let children: Vec<Box<dyn Layout>> = widths
            .into_iter()
            .map(|i| EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(i, 0.0)))
            .map(|l| Box::new(l) as Box<dyn Layout>)
            .collect();

        let spacing = 20;
        let padding = Padding::new(24.0, 42.0, 24.0, 20.0);
        let mut layout = VerticalLayout {
            children,
            spacing,
            padding,
            ..Default::default()
        };
        layout.solve_min_constraints();
        let mut max_width = widths
            .into_iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        max_width += padding.horizontal_sum();
        dbg!(layout.constraints);
        assert_eq!(layout.constraints.min_width, max_width);
    }

    #[test]
    fn calculate_min_height() {
        let heights: [f32; 5] = [500.0, 200.0, 10.2, 20.2, 45.0];
        let children: Vec<Box<dyn Layout>> = heights
            .into_iter()
            .map(|h| EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(0.0, h)))
            .map(|l| Box::new(l) as Box<dyn Layout>)
            .collect();

        let spacing = 20;
        let padding = Padding::new(24.0, 42.0, 24.0, 20.0);
        let mut layout = VerticalLayout {
            children,
            spacing,
            padding,
            ..Default::default()
        };
        layout.solve_min_constraints();
        let space_between = (heights.len() - 1) as f32 * spacing as f32;
        let mut min_height = heights.iter().sum::<f32>();
        min_height += space_between;
        min_height += padding.vertical_sum();
        assert_eq!(layout.constraints.min_height, min_height);
    }

    #[test]
    fn overflow_error() {
        let window = Size::unit(500.0);

        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(200.0, 200.0));
        let mut root = VerticalLayout::new().add_child(child);
        root.intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(0.0),
            height: BoxSizing::Fixed(0.0),
        };

        solve_layout(&mut root, window);
        let errors = solve_layout(&mut root, window);
        assert!(matches!(
            &errors[0],
            LayoutError::Overflow {
                id: _,
                axis: OverflowAxis::CrossAxis
            }
        ));
        assert!(matches!(
            &errors[1],
            LayoutError::Overflow {
                id: _,
                axis: OverflowAxis::MainAxis
            }
        ))
    }

    #[test]
    fn cross_axis_overflow() {
        let window = Size::unit(500.0);

        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(200.0, 0.0));
        let mut root = VerticalLayout::new().add_child(child);
        root.intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(0.0),
            ..Default::default()
        };

        let errors = solve_layout(&mut root, window);
        assert!(matches!(
            &errors[0],
            LayoutError::Overflow {
                id: _,
                axis: OverflowAxis::CrossAxis
            }
        ))
    }

    #[test]
    fn main_axis_overflow() {
        let window = Size::unit(500.0);

        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(0.0, 200.0));
        let mut root = VerticalLayout::new().add_child(child);

        root.intrinsic_size = IntrinsicSize {
            height: BoxSizing::Fixed(0.0),
            ..Default::default()
        };

        let errors = solve_layout(&mut root, window);
        assert!(matches!(
            &errors[0],
            LayoutError::Overflow {
                id: _,
                axis: OverflowAxis::MainAxis
            }
        ))
    }
    #[test]
    fn include_spacing_and_padding_main_axis_overflow() {
        let window = Size::unit(500.0);

        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(0.0, 180.0));
        let mut root = VerticalLayout::new().add_child(child);
        root.spacing = 20;
        root.padding = Padding::all(20.0);
        root.intrinsic_size = IntrinsicSize {
            height: BoxSizing::Fixed(200.0),
            ..Default::default()
        };

        let errors = solve_layout(&mut root, window);
        assert!(matches!(
            &errors[0],
            LayoutError::Overflow {
                id: _,
                axis: OverflowAxis::MainAxis
            }
        ))
    }

    #[test]
    fn no_duplicate_overflow_error() {
        let window = Size::unit(500.0);

        let child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(200.0, 200.0));
        let mut root = VerticalLayout::new().add_child(child);
        root.intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(0.0),
            ..Default::default()
        };

        let errors = solve_layout(&mut root, window);

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn vertical_layout() {
        let window = Size::new(800.0, 800.0);
        let mut child_1 = VerticalLayout::new();
        let mut child_2 = VerticalLayout::new();

        child_1.intrinsic_size.width = BoxSizing::Fixed(400.0);
        child_1.intrinsic_size.height = BoxSizing::Fixed(200.0);

        child_2.intrinsic_size.width = BoxSizing::Fixed(500.0);
        child_2.intrinsic_size.height = BoxSizing::Fixed(350.0);

        let mut root = VerticalLayout::new().add_children([child_1, child_2]);

        solve_layout(&mut root, window);

        assert_eq!(root.size(), Size::new(500.0, 550.0));

        assert_eq!(root.children()[0].size(), Size::new(400.0, 200.0));

        assert_eq!(root.children()[1].size(), Size::new(500.0, 350.0));
    }

    #[test]
    fn padding_applied_when_empty() {
        let mut empty = VerticalLayout {
            padding: Padding::all(23.0),
            ..Default::default()
        };
        solve_layout(&mut empty, Size::new(200.0, 200.0));

        assert_eq!(empty.size, Size::new(23.0 * 2.0, 23.0 * 2.0));
    }

    #[test]
    fn spacing_not_applied_when_empty() {
        let mut empty = VerticalLayout {
            spacing: 50,
            ..Default::default()
        };
        solve_layout(&mut empty, Size::new(200.0, 200.0));

        assert_eq!(empty.size, Size::default());
    }

    #[test]
    fn flex_sizing() {
        let window = Size::new(800.0, 800.0);
        let mut child_1 = VerticalLayout::new();
        let mut child_2 = VerticalLayout::new();

        child_1.intrinsic_size.width = BoxSizing::Flex(1);
        child_1.intrinsic_size.height = BoxSizing::Flex(1);

        child_2.intrinsic_size.width = BoxSizing::Flex(1);
        child_2.intrinsic_size.height = BoxSizing::Flex(1);

        let mut root = VerticalLayout::new()
            .intrinsic_size(IntrinsicSize::fill())
            .add_children([child_1, child_2]);

        solve_layout(&mut root, window);

        let child_size = Size::new(800.0, 400.0);
        assert_eq!(root.size(), window);
        assert_eq!(root.children()[0].size(), child_size);
        assert_eq!(root.children()[1].size(), child_size);
    }

    #[test]
    fn flex_with_shrink() {
        let window = Size::new(800.0, 800.0);
        let padding = 24;
        let spacing = 45;

        let inner_child = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(250.0, 250.0));

        let child_1 = BlockLayout::new(inner_child).padding(Padding::all(24.0));

        let child_2 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fill());

        let mut root = VerticalLayout::new().add_child(child_1).add_child(child_2);
        root.intrinsic_size.height = BoxSizing::Flex(1);
        root.padding = Padding::all(24.0);
        root.spacing = spacing;

        solve_layout(&mut root, window);

        let mut child_1_size = Size::new(250.0, 250.0);
        child_1_size += (padding * 2) as f32;

        let mut root_size = Size::new(0.0, 800.0);
        root_size.width += child_1_size.width;
        root_size.width += (padding * 2) as f32;

        let mut child_2_size = Size {
            width: root_size.width,
            height: root_size.height,
        };
        child_2_size.height -= child_1_size.height;
        child_2_size.height -= spacing as f32;
        child_2_size.height -= (padding * 2) as f32;

        assert_eq!(root.size(), root_size);
        assert_eq!(root.children[0].size(), child_1_size);
        assert_eq!(root.children[1].size(), child_2_size);
    }

    #[test]
    fn flex_factor() {
        let window = Size::new(800.0, 400.0);
        let mut child_node_1 = VerticalLayout::new();
        let mut child_node_2 = VerticalLayout::new();

        child_node_1.intrinsic_size.width = BoxSizing::Flex(1);
        child_node_1.intrinsic_size.height = BoxSizing::Flex(1);

        child_node_2.intrinsic_size.width = BoxSizing::Flex(3);
        child_node_2.intrinsic_size.height = BoxSizing::Flex(3);

        let mut node = VerticalLayout::new()
            .intrinsic_size(IntrinsicSize::fill())
            .add_children([child_node_1, child_node_2]);

        solve_layout(&mut node, window);

        let flex_1_height = 1.0 / 4.0 * window.height;
        // The two children should both be half the size
        assert_eq!(
            node.children()[0].size(),
            Size::new(window.width, flex_1_height)
        );
        assert_eq!(
            node.children()[0].size().width,
            node.children()[1].size().width,
        );
        assert_eq!(
            node.children()[1].size().height,
            3.0 * node.children()[0].size().height
        );
        assert_ne!(
            node.children()[1].size().width,
            3.0 * node.children()[0].size().width
        );
    }

    #[test]
    fn start_alignment() {
        let window = Size::new(200.0, 200.0);

        let padding = Padding::all(32.0);
        let spacing = 10;

        let child_1 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(240.0, 40.0));

        let child_2 = EmptyLayout::new().intrinsic_size(IntrinsicSize {
            width: BoxSizing::Fixed(20.0),
            ..Default::default()
        });

        let mut root = VerticalLayout {
            position: Position { x: 250.0, y: 10.0 },
            spacing,
            padding,
            children: vec![Box::new(child_1), Box::new(child_2)],
            ..Default::default()
        };

        solve_layout(&mut root, window);

        let mut child_1_pos = root.position;
        child_1_pos += padding.top;
        let mut child_2_pos = child_1_pos;
        child_2_pos.y += root.children[0].size().height + spacing as f32;

        assert_eq!(root.children[0].position(), child_1_pos);
        assert_eq!(root.children[1].position(), child_2_pos);
    }

    #[test]
    fn end_alignment() {
        let window = Size::new(200.0, 200.0);

        let padding = Padding::all(32.0);
        let spacing = 10;

        let child_1 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(240.0, 40.0));

        let child_2 = EmptyLayout::new().intrinsic_size(IntrinsicSize {
            width: BoxSizing::Fixed(20.0),
            ..Default::default()
        });

        let mut root = VerticalLayout {
            position: Position { x: 250.0, y: 10.0 },
            spacing,
            padding,
            children: vec![Box::new(child_1), Box::new(child_2)],
            main_axis_alignment: AxisAlignment::End,
            cross_axis_alignment: AxisAlignment::End,
            ..Default::default()
        };

        solve_layout(&mut root, window);

        let mut child_2_pos = Position {
            x: root.position.x + root.size.width,
            y: root.position.y + root.size.height,
        };
        child_2_pos -= padding.right;

        let mut child_1_pos = child_2_pos;
        child_1_pos.y -= root.children[1].size().height - spacing as f32;

        assert_eq!(root.children[0].position(), child_1_pos);
        assert_eq!(root.children[1].position(), child_2_pos);
    }
}
