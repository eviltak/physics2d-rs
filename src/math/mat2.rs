use std::ops::{Mul, MulAssign};
use super::Vec2;

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
    /// # use physics2d::Mat2;
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
    /// # use physics2d::Mat2;
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
        a00: 1.0,
        a01: 0.0,
        a10: 0.0,
        a11: 1.0,
    };
}

impl Mul for Mat2 {
    type Output = Mat2;
    
    fn mul(self, other: Mat2) -> Mat2 {
        &self * &other
    }
}

impl<'a> Mul<Mat2> for &'a Mat2 {
    type Output = Mat2;
    
    fn mul(self, other: Mat2) -> Mat2 {
        self * &other
    }
}

impl<'b> Mul<&'b Mat2> for Mat2 {
    type Output = Mat2;
    
    fn mul(self, other: &'b Mat2) -> Mat2 {
        &self * other
    }
}

impl<'a, 'b> Mul<&'b Mat2> for &'a Mat2 {
    type Output = Mat2;
    
    fn mul(self, other: &'b Mat2) -> Mat2 {
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
        &self * &other
    }
}

impl<'a> Mul<Vec2> for &'a Mat2 {
    type Output = Vec2;
    
    fn mul(self, other: Vec2) -> Vec2 {
        self * &other
    }
}

impl<'b> Mul<&'b Vec2> for Mat2 {
    type Output = Vec2;
    
    fn mul(self, other: &'b Vec2) -> Vec2 {
        &self * other
    }
}

impl<'a, 'b> Mul<&'b Vec2> for &'a Mat2 {
    type Output = Vec2;
    
    fn mul(self, other: &'b Vec2) -> Vec2 {
        Vec2::new(self.a00 * other.x + self.a01 * other.y,
                  self.a10 * other.x + self.a11 * other.y)
    }
}


impl MulAssign for Mat2 {
    fn mul_assign(&mut self, other: Mat2) {
        *self = *self * other;
    }
}

impl<'b> MulAssign<&'b Mat2> for Mat2 {
    fn mul_assign(&mut self, other: &'b Mat2) {
        *self = *self * other;
    }
}
