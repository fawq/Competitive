def fib(number: int, modulo: int) -> int:
    if number == 0:
        return 0
    elif number == 1:
        return 1
    else:
        return (fib(number - 1, modulo) + fib(number - 2, modulo)) % modulo
