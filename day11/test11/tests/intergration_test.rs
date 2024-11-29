use test11;


// tests目录下测试只会在cargo test的时候执行，所以不需要#[cfg(test)]
#[test]
fn it_test1() {
    assert_eq!(test11::add(2, 2),4);
}
