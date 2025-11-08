use crate::EmptyLayout;

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
    id: u32,
    kind: LayoutKind,
    intrinsic_width: IntrinsicValue,
    intrinsic_height: IntrinsicValue,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn solve_layout_from_desc(desc: LayoutDesc){
    dbg!(desc);
}

fn to_empty_layout(desc: LayoutDesc){
    dbg!(desc);
    let a = EmptyLayout::new();
}
