use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

/// A 2-dimensional vector.
///
/// The `Vec2` type can be used to represent anything that has two dimensions: a size, a point, a velocity, etc.
///
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec2 {
    /// X coordinate of the vector.
    pub x: f32,
    /// Y coordinate of the vector.
    pub y: f32,
}

impl Vec2 {
    /// Creates a new vector from its coordinates.
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
    
    /// Returns the length (magnitude) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use physics2d::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert_eq!(v.len(), 5.0);
    /// ```
    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }
    
    /// Returns the _square_ of the length (magnitude) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use physics2d::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert_eq!(v.sqr_len(), 25.0);
    /// assert_eq!(v.sqr_len(), v.len() * v.len());
    /// ```
    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    
    /// Returns the dot product of this vector with another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use physics2d::Vec2;
    /// let a = Vec2::new(3.0, 4.0);
    /// let b = Vec2::new(4.0, 5.0);
    ///
    /// assert_eq!(a.dot(&b), 32.0);
    /// ```
    #[inline]
    pub fn dot(&self, b: &Vec2) -> f32 {
        self.x * b.x + self.y * b.y
    }
    
    /// Returns the normalized (unit) vector for the given vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use physics2d::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// let l = v.len();
    /// let n = v.normalized();
    ///
    /// assert_eq!(n, v / l);
    /// assert_eq!(n.len(), 1.0);
    /// ```
    #[inline]
    pub fn normalized(self) -> Vec2 {
        let len = self.len();
        if len == 0.0 {
            Vec2::ZERO
        } else {
            self / len
        }
    }
    
    /// Returns a vector whose components are the minimum of the corresponding components
    /// of the two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use physics2d::Vec2;
    /// let a = Vec2::new(3.0, 40.0);
    /// let b = Vec2::new(40.0, 3.0);
    ///
    /// assert_eq!(a.min(&b), Vec2::new(3.0, 3.0));
    /// ```
    #[inline]
    pub fn min(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.min(other.x), self.y.min(other.y))
    }
    
    /// Returns a vector whose components are the maximum of the corresponding components
    /// of the two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use physics2d::Vec2;
    /// let a = Vec2::new(3.0, 40.0);
    /// let b = Vec2::new(40.0, 3.0);
    ///
    /// assert_eq!(a.max(&b), Vec2::new(40.0, 40.0));
    /// ```
    #[inline]
    pub fn max(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.max(other.x), self.y.max(other.y))
    }
    
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    
    pub const UP: Vec2 = Vec2 { x: 0.0, y: 1.0 };
    
    pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    
    pub const DOWN: Vec2 = Vec2 { x: 0.0, y: -1.0 };
    
    pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
    
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };
}

/// The vector cross product.
pub trait Cross<RHS = Self> {
    /// The type of the result of the cross product.
    type Output;
    
    /// Performs the cross product.
    fn cross(self, other: RHS) -> Self::Output;
}

impl Cross for Vec2 {
    type Output = f32;
    
    fn cross(self, other: Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl<'a> Cross<Vec2> for &'a Vec2 {
    type Output = f32;
    
    fn cross(self, other: Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl<'b> Cross<&'b Vec2> for Vec2 {
    type Output = f32;
    
    fn cross(self, other: &'b Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl<'a, 'b> Cross<&'b Vec2> for &'a Vec2 {
    type Output = f32;
    
    fn cross(self, other: &'b Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl Cross<f32> for Vec2 {
    type Output = Vec2;
    
    fn cross(self, s: f32) -> Vec2 {
        Vec2::new(s * self.y, -s * self.x)
    }
}

impl<'a> Cross<f32> for &'a Vec2 {
    type Output = Vec2;
    
    fn cross(self, s: f32) -> Vec2 {
        Vec2::new(s * self.y, -s * self.x)
    }
}

impl<'b> Cross<&'b f32> for Vec2 {
    type Output = Vec2;
    
    fn cross(self, s: &'b f32) -> Vec2 {
        Vec2::new(s * self.y, -s * self.x)
    }
}

impl<'a, 'b> Cross<&'b f32> for &'a Vec2 {
    type Output = Vec2;
    
    fn cross(self, s: &'b f32) -> Vec2 {
        Vec2::new(s * self.y, -s * self.x)
    }
}

impl Cross<Vec2> for f32 {
    type Output = Vec2;
    
    fn cross(self, other: Vec2) -> Vec2 {
        -other.cross(self)
    }
}

impl<'a> Cross<Vec2> for &'a f32 {
    type Output = Vec2;
    
    fn cross(self, other: Vec2) -> Vec2 {
        -other.cross(self)
    }
}

impl<'b> Cross<&'b Vec2> for f32 {
    type Output = Vec2;
    
    fn cross(self, other: &'b Vec2) -> Vec2 {
        -other.cross(self)
    }
}

impl<'a, 'b> Cross<&'b Vec2> for &'a f32 {
    type Output = Vec2;
    
    fn cross(self, other: &'b Vec2) -> Vec2 {
        -other.cross(self)
    }
}


impl Neg for Vec2 {
    type Output = Self;
    
    fn neg(self) -> Self {
        Vec2::new(-self.x, -self.y)
    }
}

impl<'a> Neg for &'a Vec2 {
    type Output = Vec2;
    
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}


impl Add for Vec2 {
    type Output = Vec2;
    
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a> Add<Vec2> for &'a Vec2 {
    type Output = Vec2;
    
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl<'b> Add<&'b Vec2> for Vec2 {
    type Output = Vec2;
    
    fn add(self, other: &'b Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a, 'b> Add<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    
    fn add(self, other: &'b Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<'b> AddAssign<&'b Vec2> for Vec2 {
    fn add_assign(&mut self, other: &'b Vec2) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


impl Sub for Vec2 {
    type Output = Vec2;
    
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl<'a> Sub<Vec2> for &'a Vec2 {
    type Output = Vec2;
    
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl<'b> Sub<&'b Vec2> for Vec2 {
    type Output = Vec2;
    
    fn sub(self, other: &'b Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl<'a, 'b> Sub<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    
    fn sub(self, other: &'b Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl<'b> SubAssign<&'b Vec2> for Vec2 {
    fn sub_assign(&mut self, other: &'b Vec2) {
        *self = Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}


impl Mul<f32> for Vec2 {
    type Output = Vec2;
    
    fn mul(self, s: f32) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl<'a> Mul<f32> for &'a Vec2 {
    type Output = Vec2;
    
    fn mul(self, s: f32) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl<'b> Mul<&'b f32> for Vec2 {
    type Output = Vec2;
    
    fn mul(self, s: &'b f32) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl<'a, 'b> Mul<&'b f32> for &'a Vec2 {
    type Output = Vec2;
    
    fn mul(self, s: &'b f32) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    
    fn mul(self, v: Vec2) -> Vec2 {
        Vec2::new(self * v.x, self * v.y)
    }
}

impl<'a> Mul<Vec2> for &'a f32 {
    type Output = Vec2;
    
    fn mul(self, v: Vec2) -> Vec2 {
        Vec2::new(self * v.x, self * v.y)
    }
}

impl<'b> Mul<&'b Vec2> for f32 {
    type Output = Vec2;
    
    fn mul(self, v: &'b Vec2) -> Vec2 {
        Vec2::new(self * v.x, self * v.y)
    }
}

impl<'a, 'b> Mul<&'b Vec2> for &'a f32 {
    type Output = Vec2;
    
    fn mul(self, v: &'b Vec2) -> Vec2 {
        Vec2::new(self * v.x, self * v.y)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, s: f32) {
        *self = Vec2 {
            x: self.x * s,
            y: self.y * s,
        };
    }
}

impl<'b> MulAssign<&'b f32> for Vec2 {
    fn mul_assign(&mut self, s: &'b f32) {
        *self = Vec2 {
            x: self.x * s,
            y: self.y * s,
        };
    }
}


impl Div<f32> for Vec2 {
    type Output = Vec2;
    
    fn div(self, s: f32) -> Vec2 {
        self * (1.0 / s)
    }
}

impl<'a> Div<f32> for &'a Vec2 {
    type Output = Vec2;
    
    fn div(self, s: f32) -> Vec2 {
        self * (1.0 / s)
    }
}

impl<'b> Div<&'b f32> for Vec2 {
    type Output = Vec2;
    
    fn div(self, s: &'b f32) -> Vec2 {
        self * (1.0 / s)
    }
}

impl<'a, 'b> Div<&'b f32> for &'a Vec2 {
    type Output = Vec2;
    
    fn div(self, s: &'b f32) -> Vec2 {
        self * (1.0 / s)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, s: f32) {
        *self *= 1.0 / s;
    }
}

impl<'b> DivAssign<&'b f32> for Vec2 {
    fn div_assign(&mut self, s: &'b f32) {
        *self *= 1.0 / s;
    }
}