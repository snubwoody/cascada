use std::fmt::Display;
use crate::Size;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// The x and y position of a layout node.
#[derive(Default, Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
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
        Self { x, y }
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
    /// use cascada::Position;
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

/// The bounds of any object that has a [`Size`] and [`Position`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Bounds {
    pub x: [f32; 2],
    pub y: [f32; 2],
}

impl Bounds {
    pub fn new(position: Position, size: Size) -> Self {
        Self {
            x: [position.x, position.x + size.width],
            y: [position.y, position.y + size.height],
        }
    }

    /// Check if a [`Position`] is within the [`Bounds`].
    ///
    /// # Example
    /// ```
    /// use cascada::{Position,Bounds,Size};
    ///
    /// let size = Size::new(250.0,100.0);
    /// let position = Position::new(10.0,0.0);
    ///
    /// let bounds = Bounds::new(position,size);
    ///
    /// assert!(bounds.within(&Position::new(50.0,45.5)));
    /// assert!(!bounds.within(&Position::new(1550.0,445.5)));
    /// ```
    pub fn within(&self, position: &Position) -> bool {
        if position.x > self.x[0]
            && position.x < self.x[1]
            && position.y > self.y[0]
            && position.y < self.y[1]
        {
            return true;
        }

        false
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<f32> for Position {
    type Output = Position;
    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<f32> for Position {
    type Output = Position;
    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.x - rhs,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<f32> for Position {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<f32> for Position {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Display for Position{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prec) = f.precision(){
            write!(f, "{:.prec$}x{:.prec$}", self.x, self.y)

        }else{
            write!(f, "{}x{}", self.x, self.y)
        }
    }
}

#[cfg(test)]
mod test{
    use std::sync::TryLockError::Poisoned;
    use super::*;

    #[test]
    fn display(){
        let pos = Position::new(5.0,35.35);
        let string = format!("{pos}");
        assert_eq!(string,"5x35.35");
    }

    #[test]
    fn display_with_precision(){
        let pos = Position::new(50.0,20.24242);
        let string = format!("{pos:.2}");
        assert_eq!(string,"50.00x20.24");
    }
}
