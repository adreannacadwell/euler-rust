#include <stdio.h>
#include <stdint.h>

static inline uint32_t even_pal(uint32_t seed) {
    uint32_t pal = seed;
    while (seed > 0) {
        pal = pal * 10u + (seed % 10u);
        seed /= 10u;
    }
    return pal;
}

static inline uint32_t odd_pal(uint32_t seed) {
    uint32_t pal = seed / 10u;
    while (seed > 0) {
        pal = pal * 10u + (seed % 10u);
        seed /= 10u;
    }
    return pal;
}

static inline int is_binary_pal(uint32_t n) {
    unsigned left = 31u - __builtin_clz(n);
    unsigned right = 0;

    while (left > right) {
        if (((n >> left) & 1u) != ((n >> right) & 1u))
            return 0;
        --left;
        ++right;
    }
    return 1;
}

int main(void) {
    uint64_t sum = 0;

    for (uint32_t seed = 1; seed < 1000; ++seed) {
        uint32_t p1 = odd_pal(seed);
        if (p1 < 1000000u && is_binary_pal(p1))
            sum += p1;

        uint32_t p2 = even_pal(seed);
        if (p2 < 1000000u && is_binary_pal(p2))
            sum += p2;
    }

    printf("%llu\n", (unsigned long long)sum);
    return 0;
}