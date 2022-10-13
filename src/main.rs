use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let mut num = input.trim_end().parse().expect("Enter a correct number!");

    loop {
        num = collatz(num);

        println!("{}", num);

        if num == 1 {
            break;
        }
    }
}

fn collatz(num: u32) -> u32 {
    if is_even(num) {
        num / 2
    } else {
        3 * num + 1
    }
}

fn is_even(num: u32) -> bool {
    if num % 2 == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even_true() {
        assert_eq!(is_even(2), true);
    }

    #[test]
    fn test_is_even_false() {
        assert_eq!(is_even(3), false);
    }

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(2), 1);
    }
}
