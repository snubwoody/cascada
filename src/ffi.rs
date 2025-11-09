use crate::{BoxSizing, EmptyLayout, GlobalId, IntrinsicSize, Layout, Position, Size, solve_layout, BlockLayout, layout};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,Default)]
#[repr(u8)]
pub enum LayoutKind {
    #[default]
    Empty = 0,
    Block = 1,
    Horizontal = 2,
    Vertical = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,Default)]
#[repr(u8)]
pub enum BoxSizingKind {
    #[default]
    Shrink = 0,
    Flex = 1,
    Fixed = 2,
}

/// A description for creating an [`IntrinsicSize`].
#[derive(Debug, Clone, Copy,Default)]
#[repr(C)]
pub struct IntrinsicValue {
    kind: BoxSizingKind,
    value: f32,
}

/// A description used for constructing a [`Layout`] node.
///
/// # Safety
/// The following can cause undefined behaviour:
/// - A description with [`LayoutKind::Empty`] but a `child_count` > 0.
/// - A description with [`LayoutKind::Block`] but a `child_count` != 1.
#[derive(Debug, Clone, Copy,Default)]
#[repr(C)]
pub struct LayoutDesc {
    id: GlobalId,
    kind: LayoutKind,
    intrinsic_width: IntrinsicValue,
    intrinsic_height: IntrinsicValue,
    child_count: usize,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LayoutNode {
    pub id: GlobalId,
    pub size: Size,
    pub position: Position,
}

/// Creates a new [`GlobalId`].
///
/// # Safety
/// This function using an [atomic counter](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU32.html)
/// internally, it is fully safe and safe to call from other threads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_global_id() -> GlobalId {
    GlobalId::new()
}

/// Constructs a [`Layout`] tree from the [layout description] and computes all
/// the sizes and positions.
///
/// # Safety
/// The caller must ensure that the types align with the equivalent type in the
/// calling language.
///
/// [layout description]: LayoutDesc
#[unsafe(no_mangle)]
pub unsafe extern "C" fn solve_layout_from_desc(descs: *const LayoutDesc,len: usize, size: Size) -> LayoutNode {
    let descs = unsafe {
        std::slice::from_raw_parts(descs,len)
    };
    let desc = descs[0];
    dbg!(descs);
    let intrinsic_size = IntrinsicSize::from_ffi(desc.intrinsic_width, desc.intrinsic_height);
    let mut layout = EmptyLayout::new()
        .set_id(desc.id)
        .intrinsic_size(intrinsic_size);
    solve_layout(&mut layout, size);
    layout.as_layout_node()
}


/// Builds a layout tree from a slice of [`LayoutDesc`].
///
/// # Panics
/// Panics if the index is out of bounds.
fn build_tree(descs: &[LayoutDesc],index: usize) -> Box<dyn Layout> {
    let desc = descs[index];
    let intrinsic_size = IntrinsicSize::from_ffi(desc.intrinsic_width, desc.intrinsic_height);
    let layout: Box<dyn Layout> = match desc.kind {
        LayoutKind::Empty => {
            let layout = EmptyLayout::new()
                .set_id(desc.id)
                .intrinsic_size(intrinsic_size);
            Box::new(layout)
        }
        LayoutKind::Block => {
            let l = build_tree(descs,index+1);
            let layout = BlockLayout::from_boxed(l)
                .set_id(desc.id)
                .intrinsic_size(intrinsic_size);
            Box::new(layout)
        }
        _ => panic!()
    };
    layout
}

impl IntrinsicSize {
    fn from_ffi(width: IntrinsicValue, height: IntrinsicValue) -> Self {
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

        Self { width, height }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn build_tree_one_node(){
        let id = GlobalId::new();
        let desc = LayoutDesc{
            id,
            kind: LayoutKind::Empty,
            ..LayoutDesc::default()
        };
        let layout = build_tree(&[desc],0);
        assert_eq!(layout.children().len(), 0);
        assert_eq!(layout.id(),id)
    }

    #[test]
    fn build_tree_nested_node(){
        let id = GlobalId::new();
        let id2 = GlobalId::new();
        let block = LayoutDesc{
            id,
            kind: LayoutKind::Block,
            child_count: 1,
            ..LayoutDesc::default()
        };
        let empty = LayoutDesc{
            id: id2,
            kind: LayoutKind::Empty,
            ..LayoutDesc::default()
        };
        let layout = build_tree(&[block, empty],0);
        assert_eq!(layout.children().len(), 1);
        assert_eq!(layout.id(),id);
        assert_eq!(layout.children()[0].id(),id2)
    }

    #[test]
    fn build_tree_2_nested_nodes(){
        let id = GlobalId::new();
        let id2 = GlobalId::new();
        let id3 = GlobalId::new();
        let block = LayoutDesc{
            id,
            kind: LayoutKind::Block,
            child_count: 1,
            ..LayoutDesc::default()
        };
        let block2 = LayoutDesc{
            id: id2,
            kind: LayoutKind::Block,
            child_count: 1,
            ..LayoutDesc::default()
        };
        let empty = LayoutDesc{
            id: id3,
            kind: LayoutKind::Empty,
            ..LayoutDesc::default()
        };
        let layout = build_tree(&[block,block2, empty],0);
        let nested_block = &layout.children()[0];
        let nested_empty = &nested_block.children()[0];
        assert_eq!(layout.children().len(), 1);
        assert_eq!(layout.id(),id);
        assert_eq!(nested_block.id(),id2);
        assert_eq!(nested_block.children().len(),1);
        assert_eq!(nested_empty.id(),id3);
    }
}
