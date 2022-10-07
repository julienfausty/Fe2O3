use infinitable::Infinitable;

// Setting values by default for commonly used types
pub type Fe2O3Int = i32;
pub type Fe2O3Float = f64;

// Type that can take infinity values for spaces that might be infinite
pub type Fe2O3SizeType = Infinitable<usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinitable() {
        let infty: Fe2O3SizeType = Fe2O3SizeType::Infinity;
        let zero: Fe2O3SizeType = Fe2O3SizeType::Finite(0);
        assert!(infty != zero, "Why would zero be equal to infinity?");
    }
}
