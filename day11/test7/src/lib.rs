
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_good() {
        println!("该信息不会输出到标准输出中");
    }
    #[test]
    fn it_works_bad() {
        println!("该信息会输出到标准输出中");
        panic!("函数发生panic")
    }
}
