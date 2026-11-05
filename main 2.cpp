#include <iostream>
using namespace std;

int main() {
    long long n = 600851475143LL;
    long long largest = 1;

    while (n % 2 == 0) {
        largest = 2;
        n /= 2;
    }

    for (long long i = 3; i * i <= n; i += 2) {
        while (n % i == 0) {
            largest = i;
            n /= i;
        }
    }

    if (n > 1) {
        largest = n;
    }

    cout << largest << endl;
    return 0;
}
