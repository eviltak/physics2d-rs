use crate::math::Vec2;

/// A 2D bounding volume in space.
///
/// Bounding volumes are often used to cheaply approximate shapes and objects. The structure of
/// a `Bounds` ensures that all operations - including expansion, intersection and overlap checks -
/// are performed quickly and efficiently.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bounds {
    /// The minimum corner point of the bounding volume. All points belonging to this volume are
    /// guaranteed to have components greater than the corresponding components of `min`.
    min: Vec2,
    /// The maximum corner point of the bounding volume. All points belonging to this volume are
    /// guaranteed to have components lesser than the corresponding components of `max`.
    max: Vec2,
}

impl Bounds {
    /// Creates a new `Bounds` from the given minimum and maximum corner points.
    pub fn new(min: Vec2, max: Vec2) -> Bounds {
        Bounds {
            min,
            max,
        }
    }
    
    /// Creates a new `Bounds` with the given center and extents. `min` and `max` of the
    /// created bounding volume are `center - extents` and `center + extents` respectively.
    ///
    /// # Example
    ///
    /// ```
    /// # use physics2d::{Bounds, Vec2};
    /// let b = Bounds::center_extents(Vec2::ONE, Vec2::new(1.0, 3.0));
    /// assert_eq!(b, Bounds::new(Vec2::new(0.0, -2.0), Vec2::new(2.0, 4.0)));
    /// ```
    pub fn center_extents(center: Vec2, extents: Vec2) -> Bounds {
        Bounds::new(center - extents, center + extents)
    }

    /// Returns the perimeter of the given bounding volume.
    ///
    /// # Example
    /// ```
    /// # use physics2d::{Bounds, Vec2};
    /// let b = Bounds::center_extents(Vec2::ONE, Vec2::new(2.0, 15.1));
    /// assert_eq!(b.perimeter(), 2.0 * 2.0 * (2.0 + 15.1));
    /// ```
    #[inline]
    pub fn perimeter(&self) -> f32 {
        2.0 * (self.max.x - self.min.x + self.max.y - self.min.y)
    }

    /// Returns the center of this bounding volume.
    #[inline]
    pub fn center(&self) -> Vec2 {
        0.5 * (self.max + self.min)
    }


    /// Returns the vector from the center to the farthest point (`max`) of the bounding volume.
    #[inline]
    pub fn extents(&self) -> Vec2 {
        0.5 * (self.max - self.min)
    }

    /// Checks whether two `Bounds` are intersecting.
    ///
    /// # Example
    ///
    /// ```
    /// # use physics2d::{Bounds, Vec2};
    /// let b1 = Bounds::center_extents(Vec2::ONE, Vec2::ONE);
    /// let b2 = Bounds::center_extents(Vec2::new(0.5, 0.5), Vec2::ONE);
    /// assert!(b1.intersects(&b2));
    /// ```
    #[inline]
    pub fn intersects(&self, other: &Bounds) -> bool {
        if self.max.x < other.min.x || self.min.x > other.max.x {
            false
        } else if self.max.y < other.min.y || self.min.y > other.max.y {
            false
        } else {
            true
        }
    }

    /// Checks whether this `Bounds` fully contains the other.
    ///
    /// # Example
    ///
    /// ```
    /// # use physics2d::{Bounds, Vec2};
    /// let b1 = Bounds::center_extents(Vec2::ONE, Vec2::ONE);
    /// let b2 = Bounds::center_extents(Vec2::new(0.5, 0.5), Vec2::new(0.5, 0.5));
    /// assert!(b1.contains(&b2));
    /// ```
    #[inline]
    pub fn contains(&self, other: &Bounds) -> bool {
        self.min.min(&other.min) == self.min && self.max.max(&other.max) == self.max
    }

    /// Returns a bounding volume that completely encompasses both the bounding volume represented
    /// by `self` and `other`.
    ///
    /// # Example
    /// ```
    /// # use physics2d::{Bounds, Vec2};
    /// let b1 = Bounds::center_extents(Vec2::ONE, Vec2::ONE);
    /// let b2 = Bounds::center_extents(Vec2::new(0.5, 0.5), Vec2::ONE);
    ///
    /// let union = Bounds::new(Vec2::new(-0.5, -0.5), Vec2::new(2.0, 2.0));
    ///
    /// assert_eq!(b1.union(&b2), union);
    ///
    /// let b3 = Bounds::new(Vec2::new(-0.5, -0.5), Vec2::new(-0.4, -0.4));
    /// let b4 = Bounds::new(Vec2::new(1.9, -0.325), Vec2::new(2.0, 2.0));
    ///
    /// assert_eq!(b3.union(&b4), union);
    /// ```
    pub fn union(&self, other: &Bounds) -> Bounds {
        Bounds::new(self.min.min(&other.min), self.max.max(&other.max))
    }

    /// Expands the `Bounds` in all directions by a `factor`.
    ///
    /// # Example
    /// ```
    /// # use physics2d::{Bounds, Vec2};
    /// # let extents = Vec2::new(0.5, 0.5);
    /// # let factor = 0.5;
    /// let b1 = Bounds::center_extents(Vec2::ONE, extents);
    /// let b2 = Bounds::center_extents(Vec2::ONE, extents * (1.0 + factor));
    ///
    /// assert_eq!(b1.expand_by(factor), b2);
    /// ```
    pub fn expand_by(&self, factor: f32) -> Bounds {
        Bounds::center_extents(self.center(), self.extents() * (1.0 + factor))
    }
}
