pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

enum Bizzare {
    Entier(usize),
    Texte(String)
}

fn fizz(n:usize)-> String{
    if n % 15 == 0 {String::from("FizzBuzz")}

    else if n % 3 == 0 {String::from("Fizz")}

    else if n % 5== 0 {String::from("Buzz")}

    else {n.to_string()}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn send_1_should_return_string_1() {
        let x = fizz(1);
        assert_eq!(x, String::from("1"));
    }

    #[test]
    fn send_3_should_return_string_fizz() {
        let x = fizz(3);
        assert_eq!(x, String::from("Fizz"));
    }

    #[test]
    fn send_6_should_return_string_fizz() {
        let x = fizz(6);
        assert_eq!(x, String::from("Fizz"));
    }

    #[test]
    fn send_9_should_return_string_fizz() {
        let x = fizz(9);
        assert_eq!(x, String::from("Fizz"));
    }

    #[test]
    fn send_5_should_return_string_buzz() {
        let x = fizz(5);
        assert_eq!(x, String::from("Buzz"));
    }

    #[test]
    fn send_10_should_return_string_buzz() {
        let x = fizz(10);
        assert_eq!(x, String::from("Buzz"));
    }

    #[test]
    fn send_15_should_return_string_fizzbuzz() {
        let x = fizz(15);
        assert_eq!(x, String::from("FizzBuzz"));
    }

    #[test]
    fn send_30_should_return_string_fizzbuzz() {
        let x = fizz(30);
        assert_eq!(x, String::from("FizzBuzz"));
    }
}
