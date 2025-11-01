use crate::GlobalId;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    OutOfBounds {
        parent_id: GlobalId,
        child_id: GlobalId,
    },
    Overflow {
        id: GlobalId,
        axis: OverflowAxis,
    },
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

impl std::error::Error for LayoutError {}

impl std::fmt::Display for LayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::OutOfBounds {
                parent_id,
                child_id,
            } => write!(
                f,
                "Widget(id:{child_id}) is out of it's parent's (id:{parent_id}) bounds"
            ),
            Self::Overflow { id, axis } => {
                write!(f, "Widget(id:{id})'s children have overflown in the {axis}")
            }
        }
    }
}
