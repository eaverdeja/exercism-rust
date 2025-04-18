use std::iter;

// This iteration has the same optimizations as the previous,
// but uses a more functional approach.
pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut primes = vec![2];

    iter::once(2_u32)
        .chain((3..).step_by(2))
        .filter(|&candidate| {
            let sqrt = candidate.isqrt();
            let is_prime = primes
                .iter()
                .take_while(|&&p| p <= sqrt)
                .all(|&p| candidate % p != 0);

            if is_prime {
                primes.push(candidate);
            }
            is_prime
        })
        .nth(n as usize)
        .unwrap()
}
