/*
Here's a recap on how Dynamic Programming (DP) works,
which is what we use to solve the 0/1 Knapsack problem.

The example:

Items: [
  { "weight": 5, "value": 10 },  // item 0
  { "weight": 4, "value": 40 },  // item 1
  { "weight": 6, "value": 30 },  // item 2
  { "weight": 4, "value": 50 }   // item 3
]

Knapsack Maximum Weight: 10

How do we maximize the value of items in the knapsack?
Well, we can use DP to efficiently solve this problem.

DP works by breaking the problem into smaller subproblems.
This way we can avoid recalculating the same subproblem multiple times.
This is essentially where the performance gain comes from.

In our case, we can model subproblems as dp[i][w],
where i is the number of items considered and w is the current weight limit.

dp[i][w] will store the maximum value that can be obtained
with the first i items and a weight limit of w.


Let's visualize the DP table:

        (empty)                         (max weight)
dp[i][w]:  0  1  2  3  4  5  6  7  8  9  10
     i=0:  0  0  0  0  0  0  0  0  0  0  0  (no items)
     i=1:  0  ?  ?  ?  ?  ?  ?  ?  ?  ?  ?  (considering item 0)
     i=2:  0  ?  ?  ?  ?  ?  ?  ?  ?  ?  ?  (considering items 0,1)
     i=3:  0  ?  ?  ?  ?  ?  ?  ?  ?  ?  ?  (considering items 0,1,2)
     i=4:  0  ?  ?  ?  ?  ?  ?  ?  ?  ?  ?  (considering all items)

---

Now let's fill in row i=1 (considering only the first item with weight 5, value 10):

- For w < 5: Can't fit item 0, so dp[1][w] = 0
- For w ≥ 5: Can fit item 0, so dp[1][w] = 10

dp[i][w]:  0  1  2  3  4  5  6  7  8  9  10
     i=0:  0  0  0  0  0  0  0  0  0  0  0
     i=1:  0  0  0  0  0 10 10 10 10 10 10

---

For row i=2 (considering items 0 and 1):

For each weight w:
- Option 1: Don't include item 1 → value = dp[1][w]
- Option 2: Include item 1 → value = dp[1][w-4] + 40 (if w ≥ 4)

We then take maximum of these options:

For example, at w=7:
- Option 1: Don't include item 1 → dp[1][7] = 10
- Option 2: Include item 1 → dp[1][7-4] + 40 = dp[1][3] + 40 = 0 + 40 = 40

dp[2][7] = max(10, 40) = 40

For w=9:
- Option 1: Don't include item 1 → dp[1][9] = 10
- Option 2: Include item 1 → dp[1][9-4] + 40 = dp[1][5] + 40 = 10 + 40 = 50

dp[2][9] = max(10, 50) = 50

The second row of the DP table looks like this:

dp[i][w]:  0  1  2  3  4  5  6  7  8  9  10
     i=0:  0  0  0  0  0  0  0  0  0  0  0
     i=1:  0  0  0  0  0 10 10 10 10 10 10
     i=2:  0  0  0  0 40 40 40 40 40 50 50

---

If we continue this process for all items, we will fill the DP table.
It ends up looking like this:

dp[i][w]:  0  1  2  3  4  5  6  7  8  9 10
     i=0:  0  0  0  0  0  0  0  0  0  0  0
     i=1:  0  0  0  0  0 10 10 10 10 10 10
     i=2:  0  0  0  0 40 40 40 40 40 50 50
     i=3:  0  0  0  0 40 40 40 40 40 50 70
     i=4:  0  0  0  0 50 50 50 50 90 90 90 <-- max value

Breaking down some key cells in row 4:

- dp[4][4] = max(dp[3][4], dp[3][0] + 50) = max(40, 0 + 50) = 50
- dp[4][7] = max(dp[3][7], dp[3][3] + 50) = max(40, 0 + 50) = 50
- dp[4][8] = max(dp[3][8], dp[3][4] + 50) = max(50, 40 + 50) = 90

The final answer in dp[4][10] = 90 indicates that the maximum value
we can get with all items and weight limit 10 is 90.

---

This approach gives us polynomial time complexity:
- O(n*w), where n is the number of items and w is the max weight

We'll do one tweak though to save up space  - we'll only keep
track of the previous row instead of the full matrix since this is what we use
when computing the current row. This gives us linear space complexity:
- O(w), where w is the max weight

The rest of the process stays the same.

Ok, with that out of the way, let's code this!
*/

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    items
        .iter()
        .fold(vec![0; (max_weight + 1) as usize], |prev_row, item| {
            (0..=max_weight)
                .map(|w| {
                    if item.weight <= w {
                        let with_item = prev_row[(w - item.weight) as usize] + item.value;
                        let without_item = prev_row[w as usize];
                        with_item.max(without_item)
                    } else {
                        prev_row[w as usize]
                    }
                })
                .collect()
        })[max_weight as usize]
}
