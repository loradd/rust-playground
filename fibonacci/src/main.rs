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
            fibonacci(sequence_index)
        ),
        Err(_) => println!("Illegal Fibonacci sequence index"),
    };
}

fn fibonacci(n: u8) -> u128 {
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

#[cfg(test)]
mod tests {

    use super::fibonacci;

    #[test]
    fn f_0() {
        assert_eq!(0, fibonacci(0));
    }

    #[test]
    fn f_1() {
        assert_eq!(1, fibonacci(1));
    }

    #[test]
    fn f_2() {
        assert_eq!(1, fibonacci(2));
    }

    #[test]
    fn f_100() {
        assert_eq!(354224848179261915075, fibonacci(100));
    }

    #[test]
    fn f_160() {
        assert_eq!(1226132595394188293000174702095995, fibonacci(160));
    }

}
