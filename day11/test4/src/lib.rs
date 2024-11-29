

fn return_bool(x:i32) -> bool {
    if x > 0 {
        true
    } else if x == 0 {
        panic!("传入的值等于0,value={}",x);
    } else {
        panic!("传入的值小于0,value={}",x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 将检查错误信息中是否有该字段
    #[should_panic(expected="传入的值等于0")]
    fn it_return_bool() {
        let x = -33;
        return_bool(x);
    }
}
