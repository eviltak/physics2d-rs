use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

// Start Vec2

#[derive(PartialEq, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline]
    pub fn len(&self) -> f32 {
        self.sqr_len().sqrt()
    }
    
    #[inline]
    pub fn sqr_len(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    
    #[inline]
    pub fn dot(&self, b: &Vec2) -> f32 {
        self.x * b.x + self.y * b.y
    }
    
    #[inline]
    pub fn normalized(self) -> Vec2 {
        self / self.len()
    }
    
    
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
    
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    
    pub const UP: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    
    pub const RIGHT: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    
    pub const DOWN: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    
    pub const LEFT: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    
    pub const ONE: Vec2 = Vec2 { x: 0.0, y: 0.0 };
}


pub trait Cross<RHS = Self> {
    type Output;
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

// End Vec2

// Start Mat2

#[derive(PartialEq, Copy, Clone)]
pub struct Mat2 {
    pub m00: f32, pub m01: f32,
    pub m10: f32, pub m11: f32,
}

impl Mat2 {
    pub fn transpose(&self) -> Mat2 {
        Mat2::new(self.m00, self.m10,
                  self.m01, self.m11)
    }
    
    
    pub fn new(m00: f32, m01: f32,
               m10: f32, m11: f32) -> Mat2 {
        Mat2 {
            m00, m01, m10, m11
        }
    }
    
    pub fn rotation(angle: f32) -> Mat2 {
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

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new(self.m00 * other.x + self.m01 * other.y,
                  self.m10 * other.x + self.m11 * other.y)
    }
}

impl MulAssign for Mat2 {
    fn mul_assign(&mut self, other: Mat2) {
        *self = *self * other;
    }
}

// End Mat2
