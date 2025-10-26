use crate::{
    AxisAlignment, BoxConstraints, BoxSizing, IntrinsicSize, Layout, LayoutError, LayoutIter,
    Padding, Position, Size,
};
use agape_core::GlobalId;

/// A [`Layout`] that arranges it's child nodes horizontally.
///
/// # Example
/// ```
/// use cascada::{EmptyLayout, HorizontalLayout, IntrinsicSize, Padding};
///
/// HorizontalLayout::new()
///     .intrinsic_size(IntrinsicSize::fill())
///     .add_child(EmptyLayout::new())
///     .padding(Padding::symmetric(10.0,20.0))
///     .spacing(12);
/// ```
#[derive(Default, Debug)]
pub struct HorizontalLayout {
    id: GlobalId,
    size: Size,
    position: Position,
    spacing: u32,
    padding: Padding,
    constraints: BoxConstraints,
    intrinsic_size: IntrinsicSize,
    /// The main axis is the axis which the content flows in, for the [`HorizontalLayout`]
    /// main axis is the `x-axis`
    main_axis_alignment: AxisAlignment,
    /// The cross axis is the `y-axis`
    cross_axis_alignment: AxisAlignment,
    children: Vec<Box<dyn Layout>>,
    errors: Vec<LayoutError>,
}

impl HorizontalLayout {
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a [`Layout`] node to the list of children.
    ///
    /// # Example
    /// ```
    /// use cascada::{HorizontalLayout,EmptyLayout,VerticalLayout};
    ///
    /// HorizontalLayout::new()
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
    /// use cascada::{HorizontalLayout,EmptyLayout};
    ///
    /// HorizontalLayout::new()
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

    /// Set this layout's [`IntrinsicSize`].
    pub fn intrinsic_size(mut self, intrinsic_size: IntrinsicSize) -> Self {
        self.intrinsic_size = intrinsic_size;
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

    /// Calculate the total minimum constraints of all
    /// the child nodes. The width is the sum of all
    /// the children's minimum width plus the space in
    /// between. The height is gotten from the largest
    /// of the minimum height.
    fn compute_children_min_size(&mut self) -> Size {
        let mut sum = Size::default();
        if self.children.is_empty() {
            return sum;
        }

        let space_between = (self.children.len() - 1) as f32 * self.spacing as f32;
        sum.width += space_between;
        for child in self.children.iter_mut() {
            let (min_width, min_height) = child.solve_min_constraints();
            sum.width += min_width;
            sum.height = sum.height.max(min_height);
        }
        sum.width += self.padding.horizontal_sum();
        sum.height += self.padding.vertical_sum();
        sum
    }

    /// Calculate the sum of the width's of all nodes with fixed sizes and the max height
    fn fixed_size_sum(&self) -> Size {
        let mut sum = Size::default();

        for (i, child) in self.children.iter().enumerate() {
            match child.get_intrinsic_size().width {
                BoxSizing::Fixed(width) => {
                    sum.width += width;
                }
                BoxSizing::Shrink => {
                    sum.width += child.constraints().min_width;
                }
                _ => {}
            }

            if let BoxSizing::Fixed(height) = child.get_intrinsic_size().height {
                sum.height = sum.height.max(height);
            }

            // Add the spacing between layouts
            if i != self.children.len() - 1 {
                sum.width += self.spacing as f32;
            }
        }

        sum
    }

    fn align_main_axis_start(&mut self) {
        let mut x_pos = self.position.x;
        x_pos += self.padding.left;

        for child in &mut self.children {
            child.set_x(x_pos);
            x_pos += child.size().width + self.spacing as f32;
        }
    }

    /// Align the children on the main axis in the center
    fn align_main_axis_center(&mut self) {
        if self.children.is_empty() {
            return;
        }

        // Sum the width of all the children.
        let mut width_sum = self
            .children
            .iter()
            .map(|child| child.size().width)
            .sum::<f32>();
        // Add the spacing in between each child
        let space_between = self.spacing * (self.children.len() - 1) as u32;
        width_sum += space_between as f32;
        let mut center_start = self.position.x + (self.size.width - width_sum) / 2.0;

        for child in &mut self.children {
            child.set_x(center_start);
            center_start += child.size().width + self.spacing as f32;
        }
    }

    fn align_main_axis_end(&mut self) {
        let mut x_pos = self.position.x + self.size.width;
        x_pos -= self.padding.right;

        for child in self.children.iter_mut().rev() {
            // Set the right edge
            x_pos -= child.size().width;
            child.set_x(x_pos);
            x_pos -= self.spacing as f32;
        }
    }

    fn align_cross_axis_start(&mut self) {
        let y = self.position.y + self.padding.top;
        for child in &mut self.children {
            child.set_y(y);
        }
    }

    fn align_cross_axis_center(&mut self) {
        for child in &mut self.children {
            let y_pos = (self.size.height - child.size().height) / 2.0 + self.position.y;
            child.set_y(y_pos);
        }
    }

    fn align_cross_axis_end(&mut self) {
        for child in &mut self.children {
            child.set_y(self.position.y + self.size.height - self.padding.bottom);
        }
    }
}

impl Layout for HorizontalLayout {
    fn id(&self) -> GlobalId {
        self.id
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

    fn position(&self) -> Position {
        self.position
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
        self.constraints.max_width = width;
    }

    fn set_min_height(&mut self, height: f32) {
        self.constraints.min_height = height;
    }

    fn set_min_width(&mut self, width: f32) {
        self.constraints.min_width = width;
    }

    fn collect_errors(&mut self) -> Vec<crate::LayoutError> {
        self.errors
            .drain(..)
            .chain(
                self.children
                    .iter_mut()
                    .flat_map(|child| child.collect_errors())
                    .collect::<Vec<_>>(),
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
                if let BoxSizing::Flex(factor) = child.get_intrinsic_size().width {
                    Some(factor)
                } else {
                    None
                }
            })
            .sum();

        let mut available_height;
        match self.intrinsic_size.height {
            BoxSizing::Shrink => available_height = self.constraints.min_height,
            BoxSizing::Fixed(_) | BoxSizing::Flex(_) => {
                available_height = self.constraints.max_height;
                available_height -= self.padding.vertical_sum();
            }
        }

        let mut available_width;
        match self.intrinsic_size.width {
            BoxSizing::Shrink => {
                available_width = self.constraints.min_width;
                available_width -= self.fixed_size_sum().width;
            }
            BoxSizing::Fixed(_) | BoxSizing::Flex(_) => {
                available_width = self.constraints.max_width;
                available_width -= self.padding.horizontal_sum();
                available_width -= self.fixed_size_sum().width;
            }
        }

        for child in &mut self.children {
            match child.get_intrinsic_size().width {
                BoxSizing::Flex(factor) => {
                    let grow_factor = factor as f32 / flex_total as f32;
                    child.set_max_width(grow_factor * available_width);
                }
                BoxSizing::Fixed(width) => {
                    child.set_max_width(width);
                }
                BoxSizing::Shrink => {
                    // Not sure about this
                    child.set_max_width(child.constraints().min_width);
                }
            }

            match child.get_intrinsic_size().height {
                BoxSizing::Flex(_) => {
                    child.set_max_height(available_height);
                }
                BoxSizing::Fixed(height) => {
                    child.set_max_height(height);
                }
                BoxSizing::Shrink => {
                    child.set_max_height(child.constraints().min_height);
                }
            }

            // Pass the max size to the children to solve their max constraints
            let space = Size {
                width: child.constraints().max_width,
                height: child.constraints().max_height,
            };

            child.solve_max_constraints(space);
        }
    }

    fn update_size(&mut self) {
        match self.intrinsic_size.width {
            BoxSizing::Flex(_) => {
                self.size.width = self.constraints.max_width;
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
            if child.position().x > self.position.x + self.size.width {
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
    use crate::{EmptyLayout, solve_layout};

    #[test]
    fn fixed_min_constraints() {
        let mut layout = HorizontalLayout {
            intrinsic_size: IntrinsicSize::fixed(20.0, 24.0),
            ..Default::default()
        };

        layout.solve_min_constraints();
        assert_eq!(layout.constraints.min_width, 20.0);
        assert_eq!(layout.constraints.min_height, 24.0);
    }

    #[test]
    fn compute_min_size_no_children() {
        let mut layout = HorizontalLayout::new();
        let size = layout.compute_children_min_size();
        assert_eq!(size, Size::default());
    }

    #[test]
    fn calculate_min_width() {
        let widths: &[f32] = &[500.0, 200.0, 10.2, 20.2, 45.0];
        let children: Vec<Box<dyn Layout>> = widths
            .iter()
            .map(|w| EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(*w, 0.0)))
            .map(|l| Box::new(l) as Box<dyn Layout>)
            .collect();

        let spacing = 20;
        let padding = Padding::new(24.0, 42.0, 24.0, 20.0);
        let mut layout = HorizontalLayout {
            children,
            spacing,
            padding,
            ..Default::default()
        };
        layout.solve_min_constraints();
        let space_between = (widths.len() - 1) as f32 * spacing as f32;
        let mut min_width = widths.iter().sum::<f32>();
        min_width += space_between;
        min_width += padding.horizontal_sum();
        assert_eq!(layout.constraints.min_width, min_width);
    }

    #[test]
    fn calculate_min_height() {
        let heights: [f32; 5] = [500.0, 200.0, 10.2, 20.2, 45.0];
        let children: Vec<Box<dyn Layout>> = heights
            .iter()
            .map(|h| EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(0.0, *h)))
            .map(|l| Box::new(l) as Box<dyn Layout>)
            .collect();

        let spacing = 20;
        let padding = Padding::new(24.0, 42.0, 24.0, 20.0);
        let mut layout = HorizontalLayout {
            children,
            spacing,
            padding,
            ..Default::default()
        };
        layout.solve_min_constraints();
        let mut max_height = heights
            .into_iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        max_height += padding.vertical_sum();
        assert_eq!(layout.constraints.min_height, max_height);
    }

    #[test]
    fn align_main_axis_center_no_children() {
        let mut layout = HorizontalLayout::new();
        layout.align_main_axis_center();
    }

    #[test]
    fn align_main_axis_end() {
        let mut child = Box::new(EmptyLayout::new());
        child.size.width = 200.0;
        let mut layout = HorizontalLayout {
            children: vec![child],
            main_axis_alignment: AxisAlignment::End,
            ..Default::default()
        };

        layout.size.width = 500.0;
        layout.position.x = 50.0;
        layout.align_main_axis_end();

        let position = layout.children[0].position();
        let right_edge = layout.size.width + layout.position.x;
        assert_eq!(position.x, right_edge - 200.0);
    }

    #[test]
    fn align_main_axis_end_include_padding() {
        let mut child = Box::new(EmptyLayout::new());
        child.size.width = 200.0;
        let mut layout = HorizontalLayout {
            children: vec![child],
            padding: Padding::new(10.0, 50.0, 20.0, 24.0),
            main_axis_alignment: AxisAlignment::End,
            ..Default::default()
        };

        layout.size.width = 500.0;
        layout.align_main_axis_end();

        let position = layout.children[0].position();
        assert_eq!(position.x, 500.0 - 200.0 - 50.0);
    }

    #[test]
    fn align_main_axis_end_multiple_children() {
        let widths: &[f32] = &[500.0, 200.0, 10.2, 20.2, 45.0];
        let children: Vec<Box<dyn Layout>> = widths
            .iter()
            .map(|w| {
                let mut layout = EmptyLayout::new();
                layout.size = Size::unit(*w);
                layout
            })
            .map(|l| Box::new(l) as Box<dyn Layout>)
            .collect();

        let size = Size::new(200.0, 200.0);
        let position = Position::new(200.0, 200.0);
        let mut layout = HorizontalLayout {
            children,
            size,
            position,
            spacing: 20,
            padding: Padding::all(24.0),
            main_axis_alignment: AxisAlignment::End,
            ..Default::default()
        };

        layout.align_main_axis_end();
        let mut right_edge = layout.size.width + layout.position.x;
        right_edge -= layout.padding.right;
        let mut iter = layout.iter();
        // Skip the root layout, we just want the children.
        iter.next();
        let layouts = iter.collect::<Vec<_>>();

        let mut x_pos = right_edge;
        for (i, l) in layouts.iter().rev().enumerate() {
            x_pos -= l.size().width;
            assert_eq!(l.position().x, x_pos, "Failed on iteration {i}");
            x_pos -= layout.spacing as f32;
        }
    }

    #[test]
    fn start_alignment() {
        let window = Size::new(200.0, 200.0);

        let padding = Padding::all(24.0);
        let spacing = 10;

        let child_1 = EmptyLayout::new().intrinsic_size(IntrinsicSize::fixed(240.0, 40.0));

        let child_2 = EmptyLayout::new().intrinsic_size(IntrinsicSize {
            width: BoxSizing::Fixed(20.0),
            ..Default::default()
        });

        let mut root = HorizontalLayout {
            position: Position { x: 250.0, y: 10.0 },
            spacing,
            padding,
            children: vec![Box::new(child_1), Box::new(child_2)],
            ..Default::default()
        };

        solve_layout(&mut root, window);

        let mut child_1_pos = root.position;
        child_1_pos.x += padding.left;
        child_1_pos.y += padding.top;
        let mut child_2_pos = child_1_pos;
        child_2_pos.x += root.children[0].size().width + spacing as f32;

        assert_eq!(root.children[0].position(), child_1_pos);
        assert_eq!(root.children[1].position(), child_2_pos);
    }
}
