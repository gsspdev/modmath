mod rifa;
mod trig;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn mul(left: usize, right: usize) -> usize {
    left * right
}

use std::ops::Mul as mu;


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn two_plus_three() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn two_times_three() { 
        let answer = mul(2, 3);
        assert_eq!(answer, 6);
    } 

    #[test]
    fn test_trig() {
        let result = trig();
        assert_eq!(result, "trigger");
    }
}
