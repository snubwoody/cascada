use std::ops::{Add, AddAssign, Sub, SubAssign};

/// The x and y position of a layout node.
#[derive(Default,Copy, Clone, PartialEq,PartialOrd,Debug)]
pub struct Position{
    pub x: f32,
    pub y: f32
}

impl Position{
    /// Create a new [`Position`].
    ///
    /// # Example
    /// ```
    /// use cascada::Position;
    ///
    /// let position  = Position::new(20.0,15.0);
    ///
    /// assert_eq!(position.x,20.0);
    /// assert_eq!(position.y,15.0);
    /// ```
    pub const fn new(x: f32, y: f32) -> Self {
        Self{x,y}
    }

    /// Translate the position by `x` and `y` amount.
    ///
    /// # Example
    ///
    /// ```
    /// use cascada::Position;
    ///
    /// let mut position = Position::new(0.0,0.0);
    /// position.translate(40.0,100.0);
    ///
    /// assert_eq!(position.x,40.0);
    /// assert_eq!(position.y,100.0);
    /// ```
    pub const fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }


    /// Create a [`Position`] with the same x and y value.
    /// 
    /// # Example
    /// ```
    /// use agape_core::Position;
    ///
    /// let position = Position::unit(500.0);
    ///
    /// assert_eq!(position.x,position.y);
    /// assert_eq!(position.x,500.0);
    /// ```
    pub fn unit(value: f32) -> Self {
        Self { x: value, y: value }
    }
}

impl Add for Position{
    type Output = Position;
    
    fn add(self, rhs: Self) -> Self {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }    
    }
}

impl Sub for Position{
    type Output = Position;
    
    fn sub(self, rhs: Self) -> Self {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<f32> for Position{
    type Output = Position;
    fn add(self, rhs: f32) -> Self {
        Self{
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f32> for Position{
    type Output = Position;
    fn sub(self, rhs: f32) -> Self {
        Self{
            x: self.x - rhs,
            y: self.x - rhs,
        }
    }
}

impl AddAssign for Position{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<f32> for Position{
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign for Position{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<f32> for Position{
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}