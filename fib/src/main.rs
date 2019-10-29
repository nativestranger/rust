use std::io;

fn main() {
    let int = get_usize();
    let value = fib(int);
    println!("result: {}", value);
}

fn fib(int: usize) -> usize {
    let mut seq = vec![0,1];
    let mut next_to_last_i = 0;
    let mut last_i = 1;

    while seq.len() < int {
        let next = seq[next_to_last_i] + seq[last_i];
        seq.push(next);
        next_to_last_i += 1;
        last_i += 1;
    }

    seq[int - 1]
}

fn get_usize() -> usize {
    let mut int = String::new();

    loop {
        println!("Enter an integer");

        io::stdin().read_line(&mut int)
            .expect("Failed to read line");

        let _int: u32 = match int.trim().parse() {
            Ok(num) => {
                return num;
            },
            Err(_) => continue,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_returns_the_value_at_the_position_given() {

        assert_eq!(0, fib(1));
        assert_eq!(1, fib(2));
        assert_eq!(1, fib(3));
        assert_eq!(2, fib(4));
        assert_eq!(3, fib(5));
        assert_eq!(5, fib(6));
        assert_eq!(8, fib(7));
        assert_eq!(13, fib(8));
        assert_eq!(21, fib(9));
        assert_eq!(34, fib(10));
    }
}
