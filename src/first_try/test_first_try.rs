#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::first_try::first_try::fib;

    #[test]
    fn test_fib() {
        let mut args = Vec::new();
        let mut out_args = Vec::new();

        let in_content = read_to_string("src/first_try/metadata/in.txt").unwrap();
        for line in in_content.lines() {
            args.append(&mut line.split_whitespace().collect());
        }

        let out_content = read_to_string("src/first_try/metadata/out.txt").unwrap();
        for line in out_content.lines() {
            out_args.append(&mut line.split_whitespace().collect());
        }

        let number: u128 = args[0].parse().unwrap();
        let modulo: u128 = args[1].parse().unwrap();

        let result: u128 = out_args[0].parse().unwrap();

        assert_eq!(fib(number, modulo), result)
    }
}