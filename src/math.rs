use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::f32;

pub const PI: f32 = f32::consts::PI;

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
    /// # use physics2d::math::Vec2;
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
    /// # use physics2d::math::Vec2;
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
    /// # use physics2d::math::Vec2;
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
    /// # use physics2d::math::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// let l = v.len();
    /// let n = v.normalized();
    ///
    /// assert_eq!(n, v / l);
    /// assert_eq!(n.len(), 1.0);
    /// ```
    #[inline]
    pub fn normalized(self) -> Vec2 {
        self / self.len()
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

impl Cross<f32> for Vec2 {
    type Output = Vec2;
    
    fn cross(self, s: f32) -> Vec2 {
        Vec2::new(s * self.y, -s * self.x)
    }
}

impl Cross<Vec2> for f32 {
    type Output = Vec2;
    
    fn cross(self, other: Vec2) -> Vec2 {
        -other.cross(self)
    }
}


impl Neg for Vec2 {
    type Output = Self;
    
    fn neg(self) -> Self {
        Vec2::new(-self.x, -self.y)
    }
}


impl Add for Vec2 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


impl Sub for Vec2 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}


impl Mul<f32> for Vec2 {
    type Output = Self;
    
    fn mul(self, s: f32) -> Self {
        Vec2::new(self.x * s, self.y * s)
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


impl Div<f32> for Vec2 {
    type Output = Self;
    
    fn div(self, s: f32) -> Self {
        self * (1.0 / s)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, s: f32) {
        *self *= 1.0 / s;
    }
}


/// A square matrix of order 2.
///
/// It is primarily used to represent 2D rotation matrices, but can be used otherwise too.
///
/// The elements are named based on their zero-based row-column positions.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Mat2 {
    /// The element of the 1st row and 1st column.
    pub a00: f32,
    /// The element of the 1st row and 2nd column.
    pub a01: f32,
    /// The element of the 2nd row and 1st column.
    pub a10: f32,
    /// The element of the 2nd row and 2nd column.
    pub a11: f32,
}

impl Mat2 {
    /// Creates a new matrix from the given elements.
    pub fn new(a00: f32, a01: f32,
               a10: f32, a11: f32) -> Mat2 {
        Mat2 {
            a00,
            a01,
            a10,
            a11
        }
    }
    
    /// Creates a new rotation matrix for a rotation angle in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// # use physics2d::math::Mat2;
    ///
    /// let a = Mat2::rotation(0.0);
    /// let b = Mat2::new(1.0, 0.0,
    ///                   0.0, 1.0);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn rotation(angle: f32) -> Mat2 {
        let (sin, cos) = angle.sin_cos();
        
        Mat2::new(cos, -sin,
                  sin, cos)
    }
    
    /// Returns the transpose of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use physics2d::math::Mat2;
    ///
    /// let a = Mat2::new(3.0, 1.0,
    ///                   8.0, 1.0);
    /// let a_t = Mat2::new(3.0, 8.0,
    ///                     1.0, 1.0);
    ///
    /// assert_eq!(a_t, a.transpose());
    /// ```
    pub fn transpose(&self) -> Mat2 {
        Mat2::new(self.a00, self.a10,
                  self.a01, self.a11)
    }
    
    pub const I: Mat2 = Mat2 {
        a00: 1.0, a01: 0.0,
        a10: 0.0, a11: 1.0,
    };
}

impl Mul for Mat2 {
    type Output = Mat2;
    
    fn mul(self, other: Mat2) -> Mat2 {
        Mat2::new(
            self.a00 * other.a00 + self.a01 * other.a10,
            self.a00 * other.a01 + self.a01 * other.a11,
            self.a10 * other.a00 + self.a11 * other.a10,
            self.a10 * other.a01 + self.a11 * other.a11
        )
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new(self.a00 * other.x + self.a01 * other.y,
                  self.a10 * other.x + self.a11 * other.y)
    }
}

impl MulAssign for Mat2 {
    fn mul_assign(&mut self, other: Mat2) {
        *self = *self * other;
    }
}
