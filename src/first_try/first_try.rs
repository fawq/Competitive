pub fn fib(number: u128, modulo: u128) -> u128 {
    if number == 0 {
        return 0;
    }
    else if number == 1 {
        return 1;
    }
    else {
        return (fib(number - 1, modulo) + fib(number - 2, modulo)) % modulo;
    }
}