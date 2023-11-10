use bevy::prelude::*;

#[derive(Component)]
pub struct Planet {}

#[derive(Component, Default)]
pub struct Mass {
    pub mass: f32,
}

#[derive(Component)]
pub struct Asteroid {}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct AngularVelocity {
    pub velocity: f32,
}

/// A `struct` containing the minimum and maximum distances to a central body.
#[derive(Component, Default)]
pub struct Orbit {
    /// The minimum distance to the central body
    pub r_min: f32,
    /// The maximum distance to the central body
    pub r_max: f32,
}

impl Orbit {
    /// Determine if an orbit is elliptical from its eccentricity.
    ///
    /// An orbit is elliptical if 0 < &epsi; < 1.
    ///
    /// # Arguments
    /// * `eccentricity` - the eccentricity of a trajectory
    ///
    /// # Examples
    /// ```rust
    /// let circular = Orbit {
    ///     r_min: 10f32,
    ///     r_max: 10f32,
    ///  };
    /// let elliptical = Orbit {
    ///     r_min: 3.0,
    ///     r_max: 5.0,
    /// };
    /// let parabolic = Orbit {
    ///     r_min: 0f32,
    ///     r_max: 10f32,
    /// };
    ///
    /// let circular_not_elliptical = !circular.is_elliptical();
    /// let elliptical = elliptical.is_elliptical();
    /// let parabolic_not_elliptical = !parabolic.is_elliptical();
    ///
    /// assert!(circular_not_elliptical);
    /// assert!(elliptical);
    /// assert!(parabolic_not_elliptical);
    /// ```
    pub fn is_elliptical(&self) -> bool {
        let e = self.eccentricity();
        0f32 < e && e < 1f32
    }

    /// Returns the eccentricity &epsi; of the orbit.
    ///
    /// # Arguments
    /// * `r_min` - the minimum distance to the body
    /// * `r_max` - the maximum distance to the body
    /// # Examples
    /// ```rust
    /// let orbit = Orbit {
    ///     r_min: 3.0,
    ///     r_max: 5.0,
    /// };
    ///
    /// let eccentricity = orbit.eccentricity();
    ///
    /// assert_eq!(eccentricity, 0.25);
    /// ```
    pub fn eccentricity(&self) -> f32 {
        (self.r_max - self.r_min) / (self.r_max + self.r_min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eccentricity_is_correct() {
        // Arrange
        let orbit = Orbit {
            r_min: 3.0,
            r_max: 5.0,
        };

        // Act
        let eccentricity = orbit.eccentricity();

        // Assert
        assert_eq!(eccentricity, 0.25);
    }

    #[test]
    fn is_elliptical_is_correct() {
        // Arrange
        let circular = Orbit {
            r_min: 10f32,
            r_max: 10f32,
        };
        let elliptical = Orbit {
            r_min: 3.0,
            r_max: 5.0,
        };

        let parabolic = Orbit {
            r_min: 0f32,
            r_max: 10f32,
        };

        // Act
        let circular_not_elliptical = !circular.is_elliptical();
        let elliptical = elliptical.is_elliptical();
        let parabolic_not_elliptical = !parabolic.is_elliptical();

        // Assert
        assert!(circular_not_elliptical);
        assert!(elliptical);
        assert!(parabolic_not_elliptical);
    }
}
