#include <cstdio>
using namespace std;

const long long LMT = 1e15;
long long ans = 0;
long long calc(long long n, long long d) {
  ++n;
  long long ret = 0, pow = 1;
  for (; pow < n; pow *= 10) {
    int c = n / pow % 10;
    ret += n / pow / 10 * pow;
    if (c == d)
      ret += n % (10 * pow) - d * pow;
    else if (c > d)
      ret += pow;
  }
  return ret;
}

void dfs(int digit, long long n, long long pow) {
  long long lo = calc(n      , digit);
  long long hi = calc(n + pow, digit);
  if (hi < n || lo > n + pow)
    return;
  if (pow == 1) {
    if (lo == n) {
      printf("=> a solution     d = %d: %lld\n", digit, n);
      ans += n;
    }
    return;
  }

  pow /= 10;
  for (int i = 0; i < 10; ++i)
    dfs(digit, n + i * pow, pow);
}

int main() {
  for (int i = 1; i <= 9; ++i)
    dfs(i, 0, LMT);
  printf("answer = %lld\n", ans);
  return 0;
}
