#[cfg(feature = "feature_add")]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(feature = "feature_minus")]
pub fn minus(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {

    use crate::minus;
    use crate::add;

    #[cfg(feature = "feature_add")]
    #[test]
    fn test_add() {
        let actual = add(2, 2);
        assert_eq!(actual, 4);
    }

    #[cfg(feature = "feature_minus")]
    #[test]
    fn test_minus() {
        let actual = minus(2, 2);
        assert_eq!(actual, 0);
    }
}