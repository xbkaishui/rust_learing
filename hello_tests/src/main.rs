fn main() {
    println!("Hello, world!");
}

// test case demo
#[cfg(test)]
mod tests {
    #[test]
    fn exploration(){
        assert_eq!(2+2, 4);
    }
    #[test]
    fn fail_test(){
        panic!("fail test");
    }

    #[test]
    fn test_mul(){
        let t = 2 * 8 ; 
        assert_eq!(t, 16);
    }
}