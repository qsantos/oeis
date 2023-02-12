#include <assert.h>
#include <stdint.h>
#include <stdio.h>

typedef __int128 int128_t;

const uint64_t small_primes[] = {
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
};

int is_counter_example(int128_t n)
{
    for (size_t i = 0; i < sizeof(small_primes) / sizeof(small_primes[0]); i++)
    {
        if (n % small_primes[i] == 0)
        {
            return 0;
        }
    }
    int128_t r = 1;
    int128_t p = n;
    int128_t acc = 2;
    while (p > 0)
    {
        if (p % 2 != 0)
        {
            r *= acc;
            r %= n;
        }
        acc *= acc;
        acc %= n;
        p /= 2;
    }
    return r == 3;
}

int main(void)
{
    int128_t solution = 4700063497;
    assert(!is_counter_example(5));
    assert(is_counter_example(solution));
    int128_t n = 1;
    while (!is_counter_example(n))
    {
        if (n % 1048576 == 1)
        {
            printf("\r%6.2f %%", (float)n / (float)solution * 100);
            fflush(stdout);
        }
        n += 4;
    }
    printf("\n");
    printf("%lu is a counter example", (uint64_t)n);
}
