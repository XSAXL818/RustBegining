pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


pub fn return_bool(num: i32) -> bool {
    if num >= 0 {
        return true
    }
    false
}

pub fn add_two(x:i32) -> i32 {
    x+3
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
    #[test]
    fn test_for_return_bool_true() {
        assert!(return_bool(22)); // 断言成功
    }
    #[test]
    fn test_for_return_bool_false() {
        assert!(!return_bool(-3));// 断言成功
    }
    #[test]
    fn test_for_add_two() {
        // 相等则测试通过
        assert_eq!(4,add_two(2));
    }

    #[test]
    fn test_for_add_two_ne() {
        // 不相等则测试通过
        assert_ne!(5,add_two(2));
    }
}
