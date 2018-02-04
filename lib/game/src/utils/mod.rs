
pub mod tmx;

pub fn epsilon(num1: f32, num2: f32, epsilon: f32) -> bool {
    return (num1 - num2).abs() <= epsilon
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_epsilon() {
        assert!(epsilon(2.0, 2.0, 0f32));
        assert!(epsilon(3.0, 3.01, 0.01));
        assert!(!epsilon(3.0, 3.01, 0.001));
    }
}