pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn return_bool(num: i32) -> bool {
    if num >= 0 {
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;// 将外部所有的模块导入

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn panic_func() {
        // panic!("测试：调用panic!");
    }
    #[test]
    fn test_for_return_bool_true() {
        assert!(return_bool(22));
    }
    #[test]
    fn test_for_return_bool_false() {
        assert!(return_bool(-3));
    }
}
