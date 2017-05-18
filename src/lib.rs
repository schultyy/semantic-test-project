#[cfg(test)]

pub fn foo() {
    println!("4223");
    println!("OC");
}

pub fn foobar() {
    println!("4223");
    println!("OC");
}

pub fn sing() {
    println!("Sing for me");
}

mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
