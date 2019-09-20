pub fn boop() {
    println!("boop");
}

#[cfg(test)]
mod tests {
    use crate_c;
    #[test]
    fn it_works() {
        crate_c::test_beep();
        assert_eq!(2 + 2, 4);
    }
}
