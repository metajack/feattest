pub fn beep() {
    println!("beep");
}

#[cfg(any(test, feature = "testing"))]
pub fn test_beep() {
    println!("test beep");
}

#[cfg(any(test, feature = "testing"))]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
