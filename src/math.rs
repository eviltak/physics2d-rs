type real = f32;

use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

// Start Vec2

#[derive(PartialEq, Copy, Clone)]
pub struct Vec2 {
    pub x: real,
    pub y: real,
}

impl Vec2 {
    #[inline]
    pub fn len(&self) -> real {
        self.sqr_len().sqrt()
    }
    
    #[inline]
    pub fn sqr_len(&self) -> real {
        self.x * self.x + self.y * self.y
    }
    
    #[inline]
    pub fn dot(&self, b: &Vec2) -> real {
        self.x * b.x + self.y * b.y
    }
    
    #[inline]
    pub fn normalized(self) -> Vec2 {
        self / self.len()
    }
    
    
    pub fn new(x: real, y: real) -> Vec2 {
        Vec2 { x, y }
    }
    
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    
    pub fn up() -> Vec2 {
        Vec2::new(0.0, 1.0)
    }
    
    pub fn right() -> Vec2 {
        Vec2::new(1.0, 0.0)
    }
    
    pub fn down() -> Vec2 {
        -Vec2::up()
    }
    
    pub fn left() -> Vec2 {
        -Vec2::down()
    }
}


pub trait Cross<RHS = Self> {
    type Output;
    fn cross(self, other: RHS) -> Self::Output;
}

impl Cross for Vec2 {
    type Output = real;
    
    fn cross(self, other: Vec2) -> real {
        self.x * other.y - self.y * other.x
    }
}

impl Cross<real> for Vec2 {
    type Output = Vec2;
    
    fn cross(self, s: real) -> Vec2 {
        Vec2::new(s * self.y, -s * self.x)
    }
}

impl Cross<Vec2> for real {
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
        *self = *self + other;
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
        *self = *self + other;
    }
}


impl Mul<real> for Vec2 {
    type Output = Self;
    
    fn mul(self, s: real) -> Self {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl MulAssign<real> for Vec2 {
    fn mul_assign(&mut self, s: real) {
        *self = *self * s;
    }
}


impl Div<real> for Vec2 {
    type Output = Self;
    
    fn div(self, s: real) -> Self {
        self * (1.0 / s)
    }
}

impl DivAssign<real> for Vec2 {
    fn div_assign(&mut self, s: real) {
        *self = *self / s;
    }
}

// End Vec2

// Start Mat2

pub struct Mat2 {
    pub m00: real, pub m01: real,
    pub m10: real, pub m11: real,
}

impl Mat2 {
    pub fn transpose(&self) -> Mat2 {
        Mat2::new(m00, m10,
                  m01, m11)
    }
    
    
    pub fn new(m00: real, m01: real,
               m10: real, m11: real) -> Mat2 {
        Mat2 {
            m00, m01, m10, m11
        }
    }
    
    pub fn rotation(angle: real) -> Mat2 {
        let (sin, cos) = angle.sin_cos();
        
        Mat2::new(cos, -sin,
                  sin, cos)
    }
}

impl Mul for Mat2 {
    type Output = Mat2;
    
    fn mul(self, other: Mat2) -> Mat2 {
        Mat2::new(self.m00 * other.m00 + self.m01 * other.m10,
                  self.m00 * other.m01 + self.m01 * other.m11,
                  self.m10 * other.m00 + self.m11 * other.m10,
                  self.m10 * other.m01 + self.m11 * other.m11)
    }
}

impl MulAssign for Mat2 {
    fn mul_assign(&mut self, other: Mat2) {
        *self = *self * other;
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new(self.m00 * other.x + self.m01 * other.y,
                  self.m10 * other.x + self.m11 * other.y)
    }
}

// End Mat2
