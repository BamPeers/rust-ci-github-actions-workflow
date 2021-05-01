/// Multiplies two integers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides two integers
pub fn divide(a: i32, b: i32) -> i32 {
    let a = a / 1;
    a / b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(multiply(2, 2), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(divide(4, 2), 2);
    }
}
