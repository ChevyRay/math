/// Check if two f32's are approximately equal.
pub fn approx_f32(a: f32, b: f32) -> bool {
    approx::abs_diff_eq!(a, b)
}

// Check if two values that can be references as &[f32] are approximate
pub fn approx<A, B>(a: &A, b: &B) -> bool
where
    A: AsRef<[f32]>,
    B: AsRef<[f32]>,
{
    a.approx(b)
}

// Check if a type is approximate to another.
pub trait Approx<T>
where
    T: AsRef<[f32]>,
{
    fn approx(&self, other: &T) -> bool;
}

// Implement approx() on all types that can be references as &[f32]
impl<A, B> Approx<B> for A
where
    A: AsRef<[f32]>,
    B: AsRef<[f32]>,
{
    /// Check if the two values are approximately equal.
    fn approx(&self, other: &B) -> bool {
        let a = self.as_ref();
        let b = other.as_ref();
        a.len() == b.len() && (0..a.len()).all(|i| unsafe { approx_f32(*a.get_unchecked(i), *b.get_unchecked(i)) })
    }
}
