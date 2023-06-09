#[cfg(feature = "feature_add")]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(feature = "feature_minus")]
pub fn minus(left: usize, right: usize) -> usize {
    left - right
}