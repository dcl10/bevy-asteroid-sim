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
    fn between(&self, lower: f32, upper: f32) -> bool {
        lower.lt(self) && upper.gt(self)
    }
}
