fn test(a: usize, b: usize) -> usize {
    if a == false as usize {
        panic!("This is an error");
    }
    if (a == b) == false {
        a + b
    } else {
        123
    }
}