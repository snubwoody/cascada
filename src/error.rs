use agape_core::GlobalId;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OverflowAxis {
    MainAxis,
    CrossAxis,
}

impl std::fmt::Display for OverflowAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum LayoutError {
    #[error("Widget(id:{child_id}) is out of it's parent's (id:{parent_id}) bounds")]
    OutOfBounds {
        parent_id: GlobalId,
        child_id: GlobalId,
    },

    #[error("Widget(id:{id})'s children have overflown in the {axis}")]
    Overflow { id: GlobalId, axis: OverflowAxis },
}

impl LayoutError {
    pub fn out_of_bound(parent_id: GlobalId, child_id: GlobalId) -> Self {
        Self::OutOfBounds {
            parent_id,
            child_id,
        }
    }

    pub fn overflow(id: GlobalId, axis: OverflowAxis) -> Self {
        Self::Overflow { id, axis }
    }
}
