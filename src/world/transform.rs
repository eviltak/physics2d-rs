use crate::math::{Vec2, Mat2};

pub struct Transform {
    pub position: Vec2,
    
    rotation: f32,
    rot_matrix: Mat2,
}

impl Transform {
    pub fn new(position: Vec2, rotation: f32) -> Transform {
        Transform {
            position,
            rotation,
            rot_matrix: Mat2::rotation(rotation),
        }
    }
    
    pub fn rotation(&self) -> f32 {
        self.rotation
    }
    
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.rot_matrix = Mat2::rotation(rotation);
    }
    
    /// Returns the world space position for the given local space position with respect to this `Transform`.
    ///
    /// The local space position is first rotated by the world space rotation matrix and then translated by the
    /// world space position.
    ///
    /// # Examples
    /// ```
    /// # use physics2d::Transform;
    /// # use physics2d::{Vec2, math};
    ///
    /// let t = Transform::new(Vec2::new(1.0, 2.0), math::PI / 2.0);
    ///
    /// assert_eq!(t.world_pos(&Vec2::ZERO), Vec2::new(1.0, 2.0));
    /// assert_eq!(t.world_pos(&Vec2::new(1.0, 1.0)), Vec2::new(0.0, 3.0));
    /// ```
    pub fn world_pos(&self, local_pos: &Vec2) -> Vec2 {
        self.rot_matrix * local_pos + self.position
    }
    
    /// Returns the world space direction for the given local space direction with respect to this `Transform`.
    ///
    /// The local space direction is rotated by the world space rotation matrix.
    ///
    /// # Examples
    /// ```
    /// # use physics2d::Transform;
    /// # use physics2d::{Vec2, math};
    ///
    /// let t = Transform::new(Vec2::new(1000.01, 333.333), math::PI / 2.0);
    ///
    /// assert!((t.world_dir(&Vec2::RIGHT) - Vec2::UP).len() < 1e-07);
    /// ```
    pub fn world_dir(&self, local_dir: &Vec2) -> Vec2 {
        self.rot_matrix * local_dir
    }
    
    /// Returns the local space position with respect to this `Transform` for the given world space position.
    ///
    /// The world space position is first brought relative to the local origin and then rotated to be relative
    /// to the local space rotation.
    ///
    /// # Examples
    /// ```
    /// # use physics2d::Transform;
    /// # use physics2d::{Vec2, math};
    ///
    /// let t = Transform::new(Vec2::new(1.0, 2.0), math::PI / 2.0);
    ///
    /// assert_eq!(t.local_pos(&Vec2::new(1.0, 2.0)), Vec2::ZERO);
    /// assert!((t.local_pos(&Vec2::new(1.0, 3.0)) - Vec2::new(1.0, 0.0)).len() < 1e-07);
    /// ```
    pub fn local_pos(&self, world_pos: &Vec2) -> Vec2 {
        self.rot_matrix.transpose() * (world_pos - self.position)
    }
    
    /// Returns the local space direction with respect to this `Transform` for the given world space direction.
    ///
    /// The world space direction is rotated to be relative to the local space rotation.
    ///
    /// # Examples
    /// ```
    /// # use physics2d::Transform;
    /// # use physics2d::{Vec2, math};
    ///
    /// let t = Transform::new(Vec2::new(1000.01, 333.333), math::PI / 2.0);
    ///
    /// assert!((t.local_dir(&Vec2::UP) - Vec2::RIGHT).len() < 1e-07);
    /// ```
    pub fn local_dir(&self, world_dir: &Vec2) -> Vec2 {
        self.rot_matrix.transpose() * world_dir
    }
}
