fn hello_world()-> String {
    println!("Hello world!");
    String::from("Hello, world!")
}



fn main() {
    hello_world();
}

// --------------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_should_return_hello_world() {
        assert_eq!(hello_world(), "Hello, world!");
    }
}