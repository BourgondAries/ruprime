#[macro_use]
extern crate text_io;

fn main() {
    let v: u32 = read![];
    println!["{}", find_prime_iterative(v)];
}

fn find_prime_iterative(initial: u32) -> u32 {
    let mut prime = 1;
    let mut curr = 1;
    let mut prime_count = 0;

    while prime_count < initial {
        for denom in 2..(curr - 1) {
            if curr % denom == 0 {
                curr = curr + 1;
                continue;
            }
        }
        prime = curr;
        curr = curr + 1;
        prime_count = prime_count + 1;
    }

    return prime;
}
