fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in b'a'..=b'z' {
        println!("{}",c);
    }
}