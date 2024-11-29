

fn return_bool(x:i32) -> bool {
    if x > 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
   fn for_return_bool() {
        let x = -3;
        assert!(return_bool(x),
        "该值小于等于0,value={}",x);
   }
}
