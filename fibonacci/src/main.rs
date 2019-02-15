use std::io;

fn main() {
    println!("Insert Fibonacci sequence index");
    let mut sequence_index: String = String::new();
    io::stdin()
        .read_line(&mut sequence_index)
        .expect("Error while reading Fibonacci sequence index");
    match sequence_index.trim().parse() {
        Ok(sequence_index) => println!(
            "Fibonacci[{}]: {}",
            sequence_index,
            fibonacci_iterative_naive(sequence_index)
        ),
        Err(_) => println!("Illegal Fibonacci sequence index"),
    };
}

fn fibonacci_iterative_naive(n: u8) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut f_n: u128 = 0;
            let mut f_n_1: u128 = 1;
            let mut f_n_2: u128 = 0;
            for _ in 1..n {
                f_n = f_n_1 + f_n_2;
                f_n_2 = f_n_1;
                f_n_1 = f_n;
            }
            f_n
        }
    }
}

#[allow(dead_code)]
fn fibonacci_recursive_naive(n: &u8) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci_recursive_naive(&(n - 1)) + fibonacci_recursive_naive(&(n - 2)),
    }
}

#[cfg(test)]
mod tests {

    use super::fibonacci_iterative_naive;

    #[test]
    fn zero_value() {
        assert_eq!(0, fibonacci_iterative_naive(0));
    }

    #[test]
    fn one_value() {
        assert_eq!(1, fibonacci_iterative_naive(1));
        assert_eq!(1, fibonacci_iterative_naive(2));
    }

    #[test]
    fn greater_value() {
        assert_eq!(354224848179261915075, fibonacci_iterative_naive(100));
        assert_eq!(
            1226132595394188293000174702095995,
            fibonacci_iterative_naive(160)
        );
    }

}
