def count(n: int) -> int:
    r = 1
    for i in range(1, 1000):
        r = (r*10) % n
        if r == 1: return i
    return 0

num = 0

for n in range(6, 1000):
    if n%2 == 0: continue
    if n%5 == 0: continue

    x = count(n)
    if x > num:
        num = x
        print(n, x)