use crate::{Layout, Size};

/// Describes the size a [`Layout`] will try to be.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub enum BoxSizing {
    /// The [`Layout`] will be a fixed size regardless of any other conditions, this can
    /// cause overflow if not used wisely.
    Fixed(f32),
    /// Tries to be as small as possible
    #[default]
    Shrink,
    /// Tries to be as big as possible, the behaviour of the flex factor is
    /// dependent on the type of layout.
    Flex(u8),
}


/// Describes the maximum and minimum size of a [`Layout`].
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct BoxConstraints {
    /// The maximum possible width.
    pub max_width: Option<f32>,
    /// The maximum possible height.
    pub max_height: f32,
    /// The minimum possible height.
    pub min_height: f32,
    /// The minimum possible width.
    pub min_width: f32,
}

impl BoxConstraints {
    /// Create new [`BoxConstraints`].
    pub const fn new() -> Self {
        Self {
            max_height: 0.0,
            max_width: None,
            min_height: 0.0,
            min_width: 0.0,
        }
    }
}

/// This is the preferred size of a [`Layout`] node.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct IntrinsicSize {
    /// The intrinsic width.
    pub width: BoxSizing,
    /// The intrinsic height.
    pub height: BoxSizing,
}

impl IntrinsicSize {
    /// Create an intrinsic size with a flex factor of `1`.
    ///
    /// # Example
    /// ```
    /// use cascada::{BoxSizing,IntrinsicSize};
    ///
    /// let instrinsic_size = IntrinsicSize::fill();
    ///
    /// assert_eq!(instrinsic_size.width,BoxSizing::Flex(1));
    /// assert_eq!(instrinsic_size.height,BoxSizing::Flex(1));
    /// ```
    pub const fn fill() -> Self {
        Self {
            width: BoxSizing::Flex(1),
            height: BoxSizing::Flex(1),
        }
    }

    /// Creates an intrinsic size with a flex factor.
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::{BoxSizing, IntrinsicSize};
    ///
    /// let intrinsic_size = IntrinsicSize::flex(8);
    ///
    /// assert_eq!(intrinsic_size.width,BoxSizing::Flex(8));
    /// assert_eq!(intrinsic_size.height,BoxSizing::Flex(8));
    /// ```
    pub const fn flex(factor: u8) -> Self {
        Self {
            width: BoxSizing::Flex(factor),
            height: BoxSizing::Flex(factor),
        }
    }

    /// Creates an [`IntrinsicSize`] that shrinks to fit its contents.
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::{BoxSizing, IntrinsicSize};
    ///
    /// let intrinsic_size = IntrinsicSize::shrink();
    ///
    /// assert_eq!(intrinsic_size.width, BoxSizing::Shrink);
    /// assert_eq!(intrinsic_size.height, BoxSizing::Shrink);
    /// ```
    pub const fn shrink() -> Self {
        Self {
            width: BoxSizing::Shrink,
            height: BoxSizing::Shrink,
        }
    }

    /// Creates an [`IntrinsicSize`] with a fixed size.
    ///
    /// # Example
    /// ```
    /// use cascada::{IntrinsicSize,BoxSizing};
    ///
    /// let intrinsic_size = IntrinsicSize::fixed(100.0,50.0);
    ///
    /// assert_eq!(intrinsic_size.width,BoxSizing::Fixed(100.0));
    /// assert_eq!(intrinsic_size.height,BoxSizing::Fixed(50.0));
    /// ```
    pub const fn fixed(width: f32, height: f32) -> Self {
        Self {
            width: BoxSizing::Fixed(width),
            height: BoxSizing::Fixed(height),
        }
    }
}

impl From<Size> for IntrinsicSize {
    fn from(size: Size) -> Self {
        IntrinsicSize {
            width: BoxSizing::Fixed(size.width),
            height: BoxSizing::Fixed(size.height),
        }
    }
}
