fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn _greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 5, "result result equal 4");
    }

    #[test]
    fn ass() {
        // let result = greeting("xx");
        assert!(false, "Hello xx?");
    }

    #[test]
    #[should_panic] // 应该出现恐慌
    fn sp() {
        panic!("is panic");
    }


    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn another2() {
        panic!("Make this test fail");
    }
}
