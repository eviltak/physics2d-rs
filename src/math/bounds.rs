use math::Vec2;

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
    
    /// Creates a new `Bounds` with the given center and extents. `extents` is the vector
    /// from the center to the farthest point (`max`) of the bounding volume. `min` and `max` of the
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
}