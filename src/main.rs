fn main() {
    println!("Hello, rust_package_use_mod!");
    inside_main();
}

// call func inside main

fn inside_main()  -> &'static str {
    return "main.rs inside_main"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_inside_main_true(){
        assert_eq!(inside_main(),"main.rs inside_main");

    }

    #[test]
    #[should_panic]
    // from here
    // https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
    fn test_inside_main_fail(){
        assert_eq!(inside_main(),"main.rs should goes failed");

    }
}
