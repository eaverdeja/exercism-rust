pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<Option<u64>> = (0..=upper_bound).map(Option::from).collect();

    (2..=upper_bound)
        .filter_map(|i| {
            // Note: if it's not prime, ? will filter it out
            let prime = primes[i as usize].take()?;

            // Mark multiples as not prime
            (prime.pow(2)..=upper_bound)
                .step_by(prime as usize)
                .for_each(|n| primes[n as usize] = None);

            Some(prime)
        })
        .collect()
}
