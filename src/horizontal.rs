use crate::{
    AxisAlignment, BoxConstraints, BoxSizing, IntrinsicSize, Layout, LayoutError, LayoutIter,
    Padding,
};
use agape_core::{GlobalId, Position, Size};

// TODO add example
/// A [`Layout`] that arranges it's children horizontally.
#[derive(Default, Debug)]
pub struct HorizontalLayout {
    pub id: GlobalId,
    pub size: Size,
    pub position: Position,
    pub spacing: u32,
    pub padding: Padding,
    pub constraints: BoxConstraints,
    pub intrinsic_size: IntrinsicSize,
    /// The main axis is the axis which the content flows in, for the [`HorizontalLayout`]
    /// main axis is the `x-axis`
    pub main_axis_alignment: AxisAlignment,
    /// The cross axis is the `y-axis`
    pub cross_axis_alignment: AxisAlignment,
    pub children: Vec<Box<dyn Layout>>,
    pub errors: Vec<LayoutError>,
}

impl HorizontalLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_child(&mut self, child: impl Layout + 'static) {
        self.children.push(Box::new(child));
    }

    pub fn add_children<I>(&mut self, children: I)
    where
        I: IntoIterator<Item: Layout + 'static>,
    {
        for child in children {
            self.children.push(Box::new(child));
        }
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

    // TODO should probably rename this function
    /// Calculate the sum of the width's of all nodes with fixed sizes and the max height
    fn fixed_size_sum(&self) -> Size {
        let mut sum = Size::default();

        for (i, child) in self.children.iter().enumerate() {
            match child.intrinsic_size().width {
                BoxSizing::Fixed(width) => {
                    sum.width += width;
                }
                BoxSizing::Shrink => {
                    sum.width += child.constraints().min_width;
                }
                _ => {}
            }

            if let BoxSizing::Fixed(height) = child.intrinsic_size().height {
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
        // TODO handle overflow
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
            // TODO handle overflow
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

    fn intrinsic_size(&self) -> IntrinsicSize {
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
                if let BoxSizing::Flex(factor) = child.intrinsic_size().width {
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
            match child.intrinsic_size().width {
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

            match child.intrinsic_size().height {
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

            // TODO not even using the space anymore
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
    use crate::EmptyLayout;

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
    fn flex_max_constraints() {
        let flex: [u8; 4] = [2, 4, 5, 1];
        let children = flex
            .into_iter()
            .map(|f| EmptyLayout {
                intrinsic_size: IntrinsicSize::flex(f),
                ..Default::default()
            })
            .map(|l| Box::new(l) as Box<dyn Layout>)
            .collect::<Vec<_>>();
        let mut layout = HorizontalLayout {
            children,
            constraints: BoxConstraints {
                max_width: 500.0,
                max_height: 250.0,
                ..Default::default()
            },
            ..Default::default()
        };

        layout.solve_max_constraints(Size::default());

        // for (layout, flex) in layout.children.iter().zip(flex.iter()) {
        //     let width = 500.0 * (*flex as f32 / flex_total as f32);
        //     // FIXME: will come back
        //     // assert_eq!(layout.constraints().max_width, width);
        //     // assert_eq!(layout.constraints().max_height, 250.0);
        // }
    }

    #[test]
    fn compute_min_size_no_children() {
        let mut layout = HorizontalLayout::new();
        let size = layout.compute_children_min_size();
        assert_eq!(size, Size::default());
    }

    #[test]
    fn calculate_min_width() {
        // TODO: test max height and width
        let widths: &[f32] = &[500.0, 200.0, 10.2, 20.2, 45.0];
        let children: Vec<Box<dyn Layout>> = widths
            .iter()
            .map(|w| EmptyLayout {
                intrinsic_size: IntrinsicSize::fixed(*w, 0.0),
                ..Default::default()
            })
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
        // TODO: test max height and width
        let heights: [f32; 5] = [500.0, 200.0, 10.2, 20.2, 45.0];
        let children: Vec<Box<dyn Layout>> = heights
            .iter()
            .map(|h| EmptyLayout {
                intrinsic_size: IntrinsicSize::fixed(0.0, *h),
                ..Default::default()
            })
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
            .map(|w| EmptyLayout {
                size: Size::unit(*w),
                ..Default::default()
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
}
