use infinitable::Infinitable;

// Setting values by default for commonly used types
type _Fe2O3Int = i32;
type _Fe2O3Float = f64;

// Type that can take infinity values for spaces that might be infinite
type _Fe2O3SizeType = Infinitable<usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinitable() {
        let infty: _Fe2O3SizeType = _Fe2O3SizeType::Infinity;
        let zero: _Fe2O3SizeType = _Fe2O3SizeType::Finite(0);
        assert!(infty != zero, "Why would zero be equal to infinity?");
    }
}
