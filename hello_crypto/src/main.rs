use blake2::{Blake2b512, Blake2s256, Digest};
use hex_literal::hex;

use std::time::{Duration, Instant};
use std::thread::sleep;

// test blake performance
fn test_blake2() {
    println!("start");
    let now = Instant::now();
    sleep(Duration::new(2, 0));
    for n in 0..1000000 {
        let mut hasher = Blake2s256::new();
        hasher.update(b"hello world");
        let res = hasher.finalize();
    }
    println!("{}", now.elapsed().as_millis());
  
    
}


fn main() {
  test_blake2();
}
