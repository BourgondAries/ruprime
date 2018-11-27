#include <iostream>
uint32_t find_prime_iterative(uint32_t initial) {
	int32_t prime = 1, curr = 1, prime_count = 0;
	while (prime_count < initial) {
		for (uint32_t denom = 2; denom < curr; ++denom) {
			if (curr % denom == 0) {
				curr += 1;
				continue;
			}
		}
		prime = curr;
		curr += 1;
		prime_count += 1;
	}
	return prime;
}
int main() {
	uint32_t v;
	std::cin >> v;
	std::cout << find_prime_iterative(v);
}
