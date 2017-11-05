type Real = f32;

use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

// Start Vec2

#[derive(PartialEq, Copy, Clone)]
pub struct Vec2 {
    pub x: Real,
    pub y: Real,
}

impl Vec2 {
    #[inline]
    pub fn len(&self) -> Real {
        self.sqr_len().sqrt()
    }
    
    #[inline]
    pub fn sqr_len(&self) -> Real {
        self.x * self.x + self.y * self.y
    }
    
    #[inline]
    pub fn dot(&self, b: &Vec2) -> Real {
        self.x * b.x + self.y * b.y
    }
    
    #[inline]
    pub fn normalized(self) -> Vec2 {
        self / self.len()
    }
    
    
    pub fn new(x: Real, y: Real) -> Vec2 {
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
    type Output = Real;
    
    fn cross(self, other: Vec2) -> Real {
        self.x * other.y - self.y * other.x
    }
}

impl Cross<Real> for Vec2 {
    type Output = Vec2;
    
    fn cross(self, s: Real) -> Vec2 {
        Vec2::new(s * self.y, -s * self.x)
    }
}

impl Cross<Vec2> for Real {
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


impl Mul<Real> for Vec2 {
    type Output = Self;
    
    fn mul(self, s: Real) -> Self {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl MulAssign<Real> for Vec2 {
    fn mul_assign(&mut self, s: Real) {
        *self = *self * s;
    }
}


impl Div<Real> for Vec2 {
    type Output = Self;
    
    fn div(self, s: Real) -> Self {
        self * (1.0 / s)
    }
}

impl DivAssign<Real> for Vec2 {
    fn div_assign(&mut self, s: Real) {
        *self = *self / s;
    }
}

// End Vec2
