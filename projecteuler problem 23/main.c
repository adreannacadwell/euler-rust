#include <stdint.h>
#include <stdlib.h>

#define LIMIT 28123

uint64_t solve_non_abundant_sums(void) {
    uint32_t *sum_div = (uint32_t *)calloc(LIMIT + 1, sizeof(uint32_t));
    if (!sum_div) return 0;

    for (uint32_t d = 1; d <= LIMIT / 2; ++d) {
        for (uint32_t m = d * 2; m <= LIMIT; m += d) {
            sum_div[m] += d;
        }
    }

    int *abundants = (int *)malloc((LIMIT + 1) * sizeof(int));
    if (!abundants) {
        free(sum_div);
        return 0;
    }

    int abundant_count = 0;
    for (int n = 1; n <= LIMIT; ++n) {
        if (sum_div[n] > (uint32_t)n) {
            abundants[abundant_count++] = n;
        }
    }

    uint8_t *can_be_written = (uint8_t *)calloc(LIMIT + 1, sizeof(uint8_t));
    if (!can_be_written) {
        free(sum_div);
        free(abundants);
        return 0;
    }

    for (int i = 0; i < abundant_count; ++i) {
        for (int j = i; j < abundant_count; ++j) {
            int s = abundants[i] + abundants[j];
            if (s > LIMIT) break;
            can_be_written[s] = 1;
        }
    }

    uint64_t result = 0;
    for (int n = 1; n <= LIMIT; ++n) {
        if (!can_be_written[n]) {
            result += (uint64_t)n;
        }
    }

    free(sum_div);
    free(abundants);
    free(can_be_written);

    return result;
}