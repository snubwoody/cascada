use std::ops::{Add, AddAssign, Sub, SubAssign};

/// The width and height of a layout node.
#[derive(Clone,Copy,PartialEq,Debug,PartialOrd,Default)]
pub struct Size{
    pub width: f32,
    pub height: f32,
}

impl Size{
    /// Create a new [`Size`].
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::Size;
    ///
    /// let size = Size::new(10.0,24.0);
    /// assert_eq!(size.width,10.0);
    /// assert_eq!(size.height,24.0);
    /// ```
    pub const fn new(width: f32,height: f32) -> Size{
        Self{width, height}
    }

    /// Create a [`Size`] with the same width and height.
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::Size;
    ///
    /// let size = Size::unit(20.0);
    /// assert_eq!(size.width,20.0);
    /// assert_eq!(size.width,size.height);
    /// ```
    pub const fn unit(value: f32) -> Size{
        Self::new(value,value)
    }
}

impl Add for Size{
    type Output = Size;

    /// Performs the `+` operation between two [`Size`]s.
    ///
    /// # Example
    /// ```
    /// use cascada::Size;
    ///
    /// let total = Size::unit(20.0) + Size::unit(30.0);
    ///
    /// assert_eq!(total,Size::unit(50.0));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Add<f32> for Size{
    type Output = Size;

    /// Add a value to both the width and height.
    ///
    /// # Example
    /// ```
    /// use cascada::Size;
    ///
    /// let total = Size::new(20.0,70.0) + 30.0;
    ///
    /// assert_eq!(total.width,50.0);
    /// assert_eq!(total.height,100.0);
    /// ```
    fn add(self, rhs: f32) -> Self::Output {
        Self{
            width: self.width + rhs,
            height: self.height + rhs,
        }
    }
}

impl Sub for Size{
    type Output = Size;

    /// Perform the `-` between two [`Size`]s.
    ///
    /// # Example
    /// ```
    /// use cascada::Size;
    ///
    /// let diff = Size::unit(100.0) - Size::unit(50.0);
    ///
    /// assert_eq!(diff,Size::unit(50.0));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl Sub<f32> for Size{
    type Output = Size;

    /// Subtract a value from both the width and height.
    ///
    /// # Example
    /// ```
    /// use cascada::Size;
    ///
    /// let total = Size::new(50.0,100.0) - 50.0;
    ///
    /// assert_eq!(total.width,0.0);
    /// assert_eq!(total.height,50.0);
    /// ```
    fn sub(self, rhs: f32) -> Self::Output {
        Self{
            width: self.width - rhs,
            height: self.height - rhs,
        }
    }
}

impl AddAssign for Size{
    /// Performs the `+=` operation on two [`Size`]s.
    ///
    /// # Example
    /// ```
    /// use cascada::Size;
    ///
    /// let mut size = Size::unit(200.0);
    /// size += Size::new(20.0,95.0);
    /// assert_eq!(size.width,220.0);
    /// assert_eq!(size.height,295.0);
    /// ```
    fn add_assign(&mut self, other: Self){
        self.width += other.width;
        self.height += other.height;
    }
}

impl AddAssign<f32> for Size{
    /// Adds a value to both the width and height.
    ///
    /// # Example
    /// ```
    /// use cascada::Size;
    ///
    /// let mut size = Size::unit(200.0);
    /// size += 50.0;
    /// assert_eq!(size.width,250.0);
    /// assert_eq!(size.height,250.0);
    /// ```
    fn add_assign(&mut self, other: f32){
        self.width += other;
        self.height += other;
    }
}

impl SubAssign for Size{
    /// Performs the `-=` operation on two [`Size`]s.
    ///
    /// # Example
    /// ```
    /// use cascada::Size;
    ///
    /// let mut size = Size::unit(100.0);
    /// size -= Size::unit(50.0);
    /// assert_eq!(size.width,50.0);
    /// assert_eq!(size.height,50.0);
    /// ```
    fn sub_assign(&mut self, other: Self){
        self.width -= other.width;
        self.height -= other.height;
    }
}

impl SubAssign<f32> for Size{
    /// Subtracts a value from both the width and height.
    ///
    /// # Example
    /// ```
    /// use cascada::Size;
    ///
    /// let mut size = Size::unit(200.0);
    /// size -= 50.0;
    /// assert_eq!(size.width,150.0);
    /// assert_eq!(size.height,150.0);
    /// ```
    fn sub_assign(&mut self, other: f32){
        self.width -= other;
        self.height -= other;
    }
}

impl From<(f32,f32)> for Size{
    /// Convert a tuple `(f32,f32)` into a [`Size`], with the
    /// first value being the width and the second being the height.
    ///
    /// # Example
    /// ```
    /// use cascada::Size;
    ///
    /// let size = Size::from((20.0f32,40.0f32));
    /// assert_eq!(size.width,20.0);
    /// assert_eq!(size.height,40.0);
    /// ```
    fn from((width,height): (f32,f32)) -> Self {
        Self{width, height}
    }
}