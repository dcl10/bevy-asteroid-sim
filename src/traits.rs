/// A trait to test if a < b < c
pub trait Between<T>
where
    T: PartialOrd,
{
    /// Returns `true` if `lower` < `self` < `upper`, else false.
    ///
    /// # Arguments
    /// * `lower` - the lower bound
    /// * `upper` - the upper bound
    fn between(&self, lower: T, upper: T) -> bool;
}

impl Between<f32> for f32 {
    /// Returns `true` if `lower` < `self` < `upper`, else false.
    ///
    /// # Arguments
    /// * `lower` - the lower bound
    /// * `upper` - the upper bound
    ///
    /// # Examples
    /// ```rust
    /// assert!(42.0.between(41.0, 43.0));
    /// assert!(!42.0.between(43.0, 44.0));
    /// assert!(!42.0.between(43.0, 41.0));
    /// ```
    fn between(&self, lower: f32, upper: f32) -> bool {
        lower.lt(self) && upper.gt(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_f32() {
        assert!(42.0.between(41.0, 43.0));
        assert!(!42.0.between(43.0, 44.0));
        assert!(!42.0.between(43.0, 41.0))
    }
}
