from collections import deque
from math import isqrt


def sieve(limit: int) -> bytearray:
    """Uses Sieve to Return a bytearray prime table where prime[n] is 1 iff n is prime."""
    prime = bytearray(b"\x01") * (limit + 1)
    prime[0:2] = b"\x00\x00"

    prime[4::2] = b"\x00" * ((limit - 4) // 2 + 1)

    for p in range(3, isqrt(limit) + 1, 2):
        if prime[p]:
            start = p * p
            step = p * 2
            prime[start:limit + 1:step] = b"\x00" * ((limit - start) // step + 1)

    return prime


def is_left_truncatable(n: int, prime: bytearray) -> bool:
    """Check whether n remains prime ..."""
    divisor = 10 ** (len(str(n)) - 1)

    while True:
        if not prime[n]:
            return False
        if divisor == 1:
            return True
        n %= divisor 
        divisor //= 10


def solve() -> int:
    limit = 1_000_000  # People said on the Internet
    prime = sieve(limit)

    results = []
    queue = deque([2, 3, 5, 7])
    extensions = (1, 3, 7, 9)

    while queue:
        p = queue.popleft()

        for d in extensions:
            q = p * 10 + d
            if q <= limit and prime[q]:
                queue.append(q)

                if q >= 10 and is_left_truncatable(q, prime):
                    results.append(q)

    return sum(results)


if __name__ == "__main__":
    print(solve())  # 748317