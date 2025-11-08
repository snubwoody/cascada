use crate::EmptyLayout;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
#[repr(C)]
pub enum LayoutKind{
    Empty = 0,
    Block = 1,
    Horizontal = 2,
    Vertical = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LayoutDesc{
    id: u32,
    kind: LayoutKind,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn solve_layout_from_desc(desc: LayoutDesc){
    dbg!(desc);
}

fn to_empty_layout(desc: LayoutDesc){
    EmptyLayout::new()
        .intrinsic_size();
}
