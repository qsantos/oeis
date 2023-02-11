def is_counter_example(n) -> bool:
    for p in [
        7,
        11,
        13,
        17,
        31,
        37,
        41,
        43,
        59,
        61,
        67,
        73,
        79,
        83,
        89,
        103,
        107,
        109,
        113,
        127,
        131,
        137,
    ]:
        if n % p == 0:
            return False
    r = 1
    p = n
    acc = 2
    while p > 0:
        if p % 2 != 0:
            r *= acc
            r %= n
        acc *= acc
        acc %= n
        p //= 2
    return r == 3


def main() -> None:
    solution = 4_700_063_497
    assert not is_counter_example(5)
    assert is_counter_example(solution)
    n = 1
    while not is_counter_example(n):
        if n % 1048576 == 1:
            print(f"\r{n / solution * 100:6.2f} %", end="", flush=True)
        n += 4
    print()
    print(f"{n} is a counter example")


if __name__ == "__main__":
    main()
