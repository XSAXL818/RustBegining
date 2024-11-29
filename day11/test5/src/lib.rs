



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_true() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("错误啦"))
        }
    }
    #[test]
    fn it_works_false() -> Result<(), String> {
        if 2+1 == 4 {
            Ok(())
        } else {
            Err(String::from("错误啦"))
        }
    }
}
