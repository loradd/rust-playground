use std::io;
use std::collections::HashMap;

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
            iterative_fibonacci(sequence_index)
        ),
        Err(_) => println!("Illegal Fibonacci sequence index"),
    };
}

fn iterative_fibonacci(n: u8) -> u128 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let (mut f_n, mut f_n_1, mut f_n_2) = (0, 1, 1);
            for _ in 2..n {
                f_n = f_n_1 + f_n_2;
                f_n_2 = f_n_1;
                f_n_1 = f_n;
            }
            f_n
        }
    }
}

fn cached_fibonacci(n : u8) -> u128 {
    let mut cache = HashMap::new();
    cache.insert(0, 0);
    cache.insert(1, 1);
    cached_fibonacci_inner(n, &mut cache)
}

fn cached_fibonacci_inner(n : u8, mut cache : &mut HashMap<u8, u128>) -> u128 {
    match cache.get(&n) {
        Some(&fibonacci_n) => fibonacci_n,
        None => {
            let fibonacci_n_1: u128 = cached_fibonacci_inner(n - 1, &mut cache);
            let fibonacci_n_2: u128 = cached_fibonacci_inner(n - 2, &mut cache);
            cache.insert(n, fibonacci_n_1 + fibonacci_n_2);
            fibonacci_n_1 + fibonacci_n_2
        }
    }
}

#[cfg(test)]
mod tests {

    use super::iterative_fibonacci;

    #[test]
    fn iterative_fibonacci_0() {
        assert_eq!(0, iterative_fibonacci(0));
    }

    #[test]
    fn iterative_fibonacci_1() {
        assert_eq!(1, iterative_fibonacci(1));
    }

    #[test]
    fn iterative_fibonacci_2() {
        assert_eq!(1, iterative_fibonacci(2));
    }

    #[test]
    fn iterative_fibonacci_100() {
        assert_eq!(354224848179261915075, iterative_fibonacci(100));
    }

    #[test]
    fn iterative_fibonacci_160() {
        assert_eq!(1226132595394188293000174702095995, cached_fibonacci(160));
    }

    use super::cached_fibonacci;

    #[test]
    fn cached_fibonacci_0() {
        assert_eq!(0, cached_fibonacci(0));
    }

    #[test]
    fn cached_fibonacci_1() {
        assert_eq!(1, cached_fibonacci(1));
    }

    #[test]
    fn cached_fibonacci_2() {
        assert_eq!(1, cached_fibonacci(2));
    }

    #[test]
    fn cached_fibonacci_100() {
        assert_eq!(354224848179261915075, cached_fibonacci(100));
    }

    #[test]
    fn cached_fibonacci_160() {
        assert_eq!(1226132595394188293000174702095995, cached_fibonacci(160));
    }

}
