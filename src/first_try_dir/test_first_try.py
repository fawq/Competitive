from src.first_try_dir.first_try import fib


def test_fib() -> None:
    args: list[str] = []
    out_args: list[str] = []

    with open("src/first_try_dir/metadata/in.txt") as input_file:
        for line in input_file.readlines():
            args.extend(line.split())

    with open("src/first_try_dir/metadata/out.txt") as output_file:
        for line in output_file.readlines():
            out_args.extend(line.split())

    number: int = int(args[0])
    modulo: int = int(args[1])

    result: int = int(out_args[0])

    assert fib(number, modulo) == result
