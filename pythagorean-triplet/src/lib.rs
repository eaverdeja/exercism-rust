/*
We'll use Euclid's formula to solve this exercise.

It states that given an arbitraty pair of integers m and n,
where m > n > 0, then:
- a = mˆ2 - nˆ2
- b = 2mn
- c = mˆ2 + nˆ2

When this is the case, [a, b, c] form a pythagorean triple.

With this information, we can generate the integer pairs for m and n,
plug them into [a, b, c] and check if that triplet satisfies
our constraint on a + b + c == sum.

---

For generating the pairs, we only check up until m=sum/3.
We can do this because:
- a is the smallest component in the triplet, so a needs to be < sum / 3
- mˆ2 - nˆ2 < sum / 3, using Euclid's formula for a
- mˆ2 < sum / 3 + 1, since n >= 1
- For this to be true, then m < sum / 3

The implementation also accounts for non-primitive triplets by introduzing
a scaling factor k.
*/

use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();

    for m in 2..(sum / 3) {
        for n in 1..m {
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;

            let mut k = 1;
            while k * (a + b + c) <= sum {
                if k * (a + b + c) == sum {
                    let mut triplet = [a, b, c].map(|t| t * k);
                    triplet.sort();
                    result.insert(triplet);
                }
                k += 1;
            }
        }
    }

    result
}
