//! Problem link: https://leetcode.com/problems/reverse-integer/
//. I can't explain it better than:
/// https://medium.com/@ManBearPigCode/how-to-reverse-a-number-mathematically-97c556626ec6
pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut reversed: i32 = 0;
        let mut n = if x.is_negative() { x.abs() } else { x };
        while n > 0 {
            let last_digit = n % 10;
            reversed = match reversed
                .checked_mul(10)
                .and_then(|val| val.checked_add(last_digit))
            {
                Some(v) => v,
                None => return 0,
            };
            n /= 10;
        }
        if x.is_negative() {
            -reversed
        } else {
            reversed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(i32::max_value()), 0);
    }
}
