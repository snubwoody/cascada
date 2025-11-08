use crate::{solve_layout, BoxSizing, EmptyLayout, GlobalId, IntrinsicSize, Layout, Position, Size};

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
#[repr(u8)]
pub enum LayoutKind{
    Empty = 0,
    Block = 1,
    Horizontal = 2,
    Vertical = 3,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
#[repr(u8)]
pub enum BoxSizingKind{
    Shrink = 0,
    Flex = 1,
    Fixed = 2,
}

/// A description for creating an [`IntrinsicSize`].
#[derive(Debug,Clone,Copy)]
#[repr(C)]
pub struct IntrinsicValue{
    kind: BoxSizingKind,
    value: f32
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LayoutDesc{
    id: GlobalId,
    kind: LayoutKind,
    intrinsic_width: IntrinsicValue,
    intrinsic_height: IntrinsicValue,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LayoutNode{
    pub id: GlobalId,
    pub size: Size,
    pub position: Position,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_global_id() -> GlobalId{
    GlobalId::new()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn solve_layout_from_desc(desc: LayoutDesc,size: Size) -> LayoutNode{
    let intrinsic_size = IntrinsicSize::from_ffi(desc.intrinsic_width, desc.intrinsic_height);
    let mut layout = EmptyLayout::new()
        .set_id(desc.id)
        .intrinsic_size(intrinsic_size);

    solve_layout(&mut layout,size);
    layout.as_layout_node()
}

impl IntrinsicSize{
    fn from_ffi(width: IntrinsicValue, height: IntrinsicValue) -> Self{
        let width = match width.kind {
            BoxSizingKind::Fixed => BoxSizing::Fixed(width.value),
            BoxSizingKind::Shrink => BoxSizing::Shrink,
            BoxSizingKind::Flex => BoxSizing::Flex(width.value as u32),
        };

        let height = match height.kind {
            BoxSizingKind::Fixed => BoxSizing::Fixed(height.value),
            BoxSizingKind::Shrink => BoxSizing::Shrink,
            BoxSizingKind::Flex => BoxSizing::Flex(height.value as u32),
        };

        Self{
            width,
            height,
        }
    }
}
