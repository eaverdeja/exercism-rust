<!-- omit in toc -->
# Solutions to the problems in the [Rust Exercism track](https://exercism.org/tracks/rust)

Below is a brief analysis of each solution. This can be used to search for common language features. Solutions are ordered alphabetically.

- [Disclaimer](#disclaimer)
- [Acronym](#acronym)
- [All-Your-Base](#all-your-base)
- [Allergies](#allergies)
- [Alphametics](#alphametics)
- [Anagram](#anagram)
- [Armstrong Numbers](#armstrong-numbers)
- [Binary Search](#binary-search)
- [Bob](#bob)
- [Bottle Song](#bottle-song)
- [Bowling](#bowling)
- [Clock](#clock)
- [Collatz Conjecture](#collatz-conjecture)
- [Difference of Squares](#difference-of-squares)
- [Dot DSL](#dot-dsl)
- [Eliud's Eggs](#eliuds-eggs)
- [ETL](#etl)
- [Gigasecond](#gigasecond)
- [Grade School](#grade-school)
- [Grains](#grains)
- [Hamming](#hamming)
- [Hello World](#hello-world)
- [High Scores](#high-scores)
- [ISBN Verifier](#isbn-verifier)
- [Isogram](#isogram)
- [Kindergarten Garden](#kindergarten-garden)
- [Leap](#leap)
- [Luhn](#luhn)
- [Matching Brackets](#matching-brackets)
- [Minesweeper](#minesweeper)
- [Nth Prime](#nth-prime)
- [Nucleotide Count](#nucleotide-count)
- [PaaS I/O](#paas-io)
- [Palindrome Products](#palindrome-products)
- [Pangram](#pangram)
- [Pascal's Triangle](#pascals-triangle)
- [Perfect Numbers](#perfect-numbers)
- [Pig Latin](#pig-latin)
- [Prime Factors](#prime-factors)
- [Proverb](#proverb)
- [Queen Attack](#queen-attack)
- [Raindrops](#raindrops)
- [Reverse String](#reverse-string)
- [RNA Transcription](#rna-transcription)
- [Run-length Encoding](#run-length-encoding)
- [Saddle Points](#saddle-points)
- [Series](#series)
- [Space Age](#space-age)
- [Sublist](#sublist)
- [Sum of Multiples](#sum-of-multiples)
- [Testing](#testing)
- [Metrics](#metrics)

## Disclaimer
> We inherently imitate to learn. None of these solutions are my own - some were
> inspired by community solutions from Exercism, others were ideas proposed by AI.
> There are even a few that were of my own devise, but even then I probably
> tapped into the collective unconscious to come up with them.
  
## [Acronym](https://github.com/eaverdeja/exercism-rust/tree/main/acronym/src/lib.rs)

- Method chaining with `split`, `filter`, `map`, `collect`
- Closures for filtering and mapping operations
- Iterator combinators (`once`, `chain`, `skip`)
- Separate function handling special cases
- Pattern matching with `char` predicates

## [All-Your-Base](https://github.com/eaverdeja/exercism-rust/tree/main/all-your-base/src/lib.rs)

- Custom `Error` enum with variants
- `Result` type for error handling
- Iterator methods (`enumerate`, `try_fold`)
- Split conversion into two phases: to/from decimal
- `try_fold` for elegant error propagation

## [Allergies](https://github.com/eaverdeja/exercism-rust/tree/main/allergies/src/lib.rs)

- `Enum` with explicit discriminator values as bit flags
- Bit manipulation with bitwise `AND` operations
- `filter_map` for conditional collection transformation
- Efficient bit checking with bitwise operations
- Using enum discriminants as bit values

## [Alphametics](https://github.com/eaverdeja/exercism-rust/tree/main/alphametics/src/lib.rs)

- Backtracking algorithm implementation
- `HashMap` and `HashSet` for efficient lookups
- Two different solving strategies for different input sizes
- Prunes invalid candidates early with partial validation
- Letter ordering optimization to improve search efficiency

## [Anagram](https://github.com/eaverdeja/exercism-rust/tree/main/anagram/src/lib.rs)

- Lifetimes for borrowed data (`'a`)
- Unicode handling with external crate
- Case-insensitive comparison with lowercase transformation
- Using `graphemes` instead of `chars` for proper Unicode support
- Direct `HashMap` comparison for anagram detection

## [Armstrong Numbers](https://github.com/eaverdeja/exercism-rust/tree/main/armstrong-numbers/src/lib.rs)

- Conversion of integer to string for digit extraction
- Iterators with `map()` and `sum()` for calculation
- Type conversion with `to_digit(10)`
- Leveraging numeric `pow()` function for exponentiation

## [Binary Search](https://github.com/eaverdeja/exercism-rust/tree/main/binary-search/src/lib.rs)

- Generic implementation with `trait` bounds (`AsRef<[T]>`, `Ord`)
- Recursive approach using slicing
- `Option` type for handling absence of value
- Pattern matching on comparison result
- `map()` to adjust indices when searching right half

## [Bob](https://github.com/eaverdeja/exercism-rust/tree/main/bob/src/lib.rs)

- Pattern matching with guard clauses
- Function composition for separation of concerns
- String manipulation (`trim()`, `ends_with()`)
- Character classification methods
- Efficient iterators with `filter()`, `all()`, `any()`

## [Bottle Song](https://github.com/eaverdeja/exercism-rust/tree/main/bottle-song/src/lib.rs)

- Range-based iteration for multiple verses
- String formatting with `format!()` macro
- String concatenation with `join()` method
- Conditional logic for singular/plural cases

## [Bowling](https://github.com/eaverdeja/exercism-rust/tree/main/bowling/src/lib.rs)

- Object-oriented design with multiple structures
- Custom `Error` enum for error handling
- `Option` for optional values and state tracking
- Fixed-size array initialization
- Method organization for clear separation of concerns

## [Clock](https://github.com/eaverdeja/exercism-rust/tree/main/clock/src/lib.rs)

- Implementation of `Display` trait for formatting
- `rem_euclid` for proper modular arithmetic
- Immutable design returning new objects
- Custom implementation of `PartialEq`
- Zero-padding in formatting with `{:02}`

## [Collatz Conjecture](https://github.com/eaverdeja/exercism-rust/tree/main/collatz-conjecture/src/lib.rs)

- Pattern matching with conditions
- Recursive function with `Option` for errors
- Guard against arithmetic overflow with `checked_*`
- Function composition with `map`

## [Difference of Squares](https://github.com/eaverdeja/exercism-rust/tree/main/difference-of-squares/src/lib.rs)

- Range iteration with inclusive ranges
- Method chaining with `sum` and `map`
- Algorithmic optimization (O(n) to O(1))

## [Dot DSL](https://github.com/eaverdeja/exercism-rust/tree/main/dot-dsl/src/lib.rs)

- Builder pattern implementation
- Module organization with submodules
- `BTreeMap`/`HashMap` for attribute storage
- Custom `trait` implementations (`Deref`, `From`, `PartialEq`)
- Method chaining (fluent interface)

## [Eliud's Eggs](https://github.com/eaverdeja/exercism-rust/tree/main/eliuds-eggs/src/lib.rs)

- Bit manipulation with shift and mask operations
- Functional approach with `filter` and `count`
- Helper function for bit counting logic

## [ETL](https://github.com/eaverdeja/exercism-rust/tree/main/etl/src/lib.rs)

- `BTreeMap` for ordered map functionality
- Flat mapping for data transformation
- Functional pipeline with iterators

## [Gigasecond](https://github.com/eaverdeja/exercism-rust/tree/main/gigasecond/src/lib.rs)

- External crate usage (`time`)
- Operator overloading
- Numeric literal with underscore for readability

## [Grade School](https://github.com/eaverdeja/exercism-rust/tree/main/grade-school/src/lib.rs)

- `BTreeMap` for sorted key storage
- `BTreeSet` for sorted unique values
- `Default` trait implementation
- Smart use of entry API for insertions

## [Grains](https://github.com/eaverdeja/exercism-rust/tree/main/grains/src/lib.rs)

- Pattern matching with ranges
- Power function with type annotation
- Functional approach with `map` and `sum`

## [Hamming](https://github.com/eaverdeja/exercism-rust/tree/main/hamming/src/lib.rs)

- Early return pattern for error handling
- Iterator chaining with `zip`, `filter`, `count`
- `Option` type for error handling
- Tuple pattern matching in `filter`

## [Hello World](https://github.com/eaverdeja/exercism-rust/tree/main/hello-world/src/lib.rs)

- String literal with static lifetime (`&'static str`)
- Basic function declaration with return type

## [High Scores](https://github.com/eaverdeja/exercism-rust/tree/main/high-scores/src/lib.rs)

- `Struct` with lifetime parameter (`'a`)
- Iterator methods (`last()`, `cloned()`, `max()`)
- Vector manipulation with `sort()`, `rev()`, `take()`
- Borrowing with lifetimes

## [ISBN Verifier](https://github.com/eaverdeja/exercism-rust/tree/main/isbn-verifier/src/lib.rs)

- String manipulation with `replace()`
- `try_fold()` for calculation with early exit
- Special case handling with conditional logic
- `enumerate()` to track position

## [Isogram](https://github.com/eaverdeja/exercism-rust/tree/main/isogram/src/lib.rs)

- `HashSet` for efficient duplicate checking
- Chained iterator methods
- Using `insert()` return value for duplicate checking
- Case-insensitive comparison with `to_ascii_lowercase()`

## [Kindergarten Garden](https://github.com/eaverdeja/exercism-rust/tree/main/kindergarten-garden/src/lib.rs)

- Constant array for student names
- `position()` to find array index by name
- Pattern matching for seed type conversion
- Complex iterator chaining

## [Leap](https://github.com/eaverdeja/exercism-rust/tree/main/leap/src/lib.rs)

- Pattern matching for different cases
- Readable approach to complex boolean logic
- Performance-optimized conditionals

## [Luhn](https://github.com/eaverdeja/exercism-rust/tree/main/luhn/src/lib.rs)

- Input validation with multiple conditions
- Advanced iterator operations
- Numeric conversion with `to_digit(10)`
- Modulo checking for algorithm validation

## [Matching Brackets](https://github.com/eaverdeja/exercism-rust/tree/main/matching-brackets/src/lib.rs)

- `HashMap` for bracket pairs
- Stack-based algorithm with `Vec` push/pop
- Pattern matching for character types
- Conditional logic with `if let`

## [Minesweeper](https://github.com/eaverdeja/exercism-rust/tree/main/minesweeper/src/lib.rs)

- 2D grid processing with string/byte conversion
- Nested mapping for coordinates
- Boundary checking logic for grid neighbors
- `as_bytes()` for performance optimization

## [Nth Prime](https://github.com/eaverdeja/exercism-rust/tree/main/nth-prime/src/lib.rs)

- Functional approach with iterator chaining
- Square root optimization for primality checking
- `step_by(2)` to skip even numbers
- Iterator methods for sequence processing

## [Nucleotide Count](https://github.com/eaverdeja/exercism-rust/tree/main/nucleotide-count/src/lib.rs)

- `HashMap` for count storage
- `try_fold` for iteration with error handling
- `Result` type for errors
- Functional collection initialization

## [PaaS I/O](https://github.com/eaverdeja/exercism-rust/tree/main/paasio/src/lib.rs)

- Wrapper types for I/O statistics tracking
- Generic types with `trait` bounds
- Implementation of `Read` and `Write` traits
- `Result.inspect()` for side effects

## [Palindrome Products](https://github.com/eaverdeja/exercism-rust/tree/main/palindrome-products/src/lib.rs)

- Custom structs for factor representation
- `HashSet` for unique factor pairs
- Square root optimization
- Skipping multiples of 10

## [Pangram](https://github.com/eaverdeja/exercism-rust/tree/main/pangram/src/lib.rs)

- Case insensitivity with `to_lowercase()`
- Functional solution with character range and `all()`
- `contains()` method for letter checking

## [Pascal's Triangle](https://github.com/eaverdeja/exercism-rust/tree/main/pascals-triangle/src/lib.rs)

- `fold()` to build rows incrementally
- Functional approach with `map()`
- Pattern matching for edge cases
- Compact code with closures

## [Perfect Numbers](https://github.com/eaverdeja/exercism-rust/tree/main/perfect-numbers/src/lib.rs)

- `Enum` for classification types
- `match` on `Ordering` for classification
- Square root optimization for factors
- `?` operator for error propagation

## [Pig Latin](https://github.com/eaverdeja/exercism-rust/tree/main/pig-latin/src/lib.rs)

- Complex string parsing with pattern matching
- `split_at()` for string manipulation
- Helper functions for concern separation
- `peekable()` iterator for lookahead

## [Prime Factors](https://github.com/eaverdeja/exercism-rust/tree/main/prime-factors/src/lib.rs)

- Loop with pattern matching
- Trial division algorithm
- Efficient value/divisor updates

## [Proverb](https://github.com/eaverdeja/exercism-rust/tree/main/proverb/src/lib.rs)

- `match` for empty list handling
- `windows(2)` for adjacent pairs
- Chained iterators for output construction
- `join()` for string concatenation

## [Queen Attack](https://github.com/eaverdeja/exercism-rust/tree/main/queen-attack/src/lib.rs)

- Custom structs for chess positions
- Range pattern matching for validation
- Algebraic properties for attack detection
- Absolute differences for diagonal checking with `abs_diff()`

## [Raindrops](https://github.com/eaverdeja/exercism-rust/tree/main/raindrops/src/lib.rs)

- Functional approach with iterators
- Array of tuples for data-driven approach
- `filter`/`map` pattern for transformation
- String concatenation with `collect()`

## [Reverse String](https://github.com/eaverdeja/exercism-rust/tree/main/reverse-string/src/lib.rs)

- `unicode_segmentation` crate for Unicode handling
- Multiple implementation approaches
- `chars().rev().collect()` pattern
- Alternative `fold`-based approach

## [RNA Transcription](https://github.com/eaverdeja/exercism-rust/tree/main/rna-transcription)

- Tuple structs with private data (`Dna(String)`, `Rna(String)`)
- Pattern matching with match for nucleotide conversion
- `try_fold` for validation with early return on error

## [Run-length Encoding](https://github.com/eaverdeja/exercism-rust/tree/main/run-length-encoding)

- `peekable()` iterator for lookahead while encoding
- Iterative approach with state tracking variables for decoding
- `parse()` with error handling for safe number conversion
- `repeat()` method for character replication during decoding

## [Saddle Points](https://github.com/eaverdeja/exercism-rust/tree/main/saddle-points)

- `iter().enumerate()` with tuple unpacking to track indices
- Precomputes column minimums for efficiency
- Combines row maxima and column minima checks by using `map()`, `min()`, `max()` methods

## [Series](https://github.com/eaverdeja/exercism-rust/tree/main/series/src/lib.rs)

- `windows()` for sliding window operations
- Efficient collection transformation
- Concise solution with chained methods

## [Space Age](https://github.com/eaverdeja/exercism-rust/tree/main/space-age/src/lib.rs)

- `const` values for orbital periods
- `trait` with default implementation
- Empty structs implementing shared trait
- `From` trait for type conversion
- Single calculation applying to all planets

## [Sublist](https://github.com/eaverdeja/exercism-rust/tree/main/sublist/src/lib.rs)

- `Enum` for comparison results
- Early returns for special cases
- Tuple destructuring for comparison direction
- `windows()` for subsequence matching
- Pattern matching on list length

## [Sum of Multiples](https://github.com/eaverdeja/exercism-rust/tree/main/sum-of-multiples/src/lib.rs)

- `HashSet` for duplicate elimination
- Functional pipeline with `filter`/`flat_map`/`collect`
- `step_by()` for multiple generation
- Zero value filtering to avoid infinite sequences

---

This list was initially generated with Claude code with the following prompt:

> This directory is composed of several small Rust projects. Can you go into each directory and check out the solution for the problem described in the README, contained in the src folder, and provide some commentary regarding the solution? Keep it brief and to the point (no flattering please), but please note any notable language features employed in the solution, or just smart choices in general. Please provide this in a markdown format, with each problem statement as a header, and commentary in a list format. Oh, and order the problems alphabetically.

The resulting list was then formatted with the following prompt:

> Can you help me make the following markdown list a bit prettier?
> 1. Each header should be prefixed with ## (for h2)
> 2. Each header should be formatted as a link to the github repo where these solutions are hosted. You can assume there's a valid link at https://github.com/eaverdeja/exercism-rust/tree/main/{kebab-case-solution-name}
> 3. Enclose any word that describes some code item in backticks. For ex. HashSet, step_by(), Enum, trait, AsRef<[T]>, function names, enum variants etc.

For subsequent generations, one could add to the prompt:

> Please look at the top-level README.md file and note solutions that are already accounted for. Do not recurse into the subdirectories for these solutions. Only annotate solutions that are not present in the top-level README.md

---

## Testing

This repo is structured as a Cargo workspace, in where each solution is a "member". This allows us to run tests for all solutions in one go with:
```console
$ cargo test --workspace --test '*'
```

This is taking ~2.7s on my M1 max 32GB.
The main offenders here are [Alphametics](#alphametics) and [Palindrome Products](#palindrome-products). Alphametics is especially brutal with its [final boss test case](https://github.com/eaverdeja/exercism-rust/blob/main/alphametics/tests/alphametics.rs#L105).


## Metrics

Here are all solutions ordered by lines of code. Use this a proxy for how complex or elegant a solution is 😄

> This breakdown is not perfect since solutions with multiple files
> are not grouped together. But alas!

```console
====================================================================================================
 Language                                 Files        Lines         Code     Comments       Blanks
====================================================================================================
 Rust                                        52         2035         1566          199          270
----------------------------------------------------------------------------------------------------
 ./alphametics/src/lib.rs                                463          289          111           63
 ./bowling/src/lib.rs                                    211          161           17           33
 ./palindrome-products/src/lib.rs                         86           73            1           12
 ./paasio/src/lib.rs                                      79           66            0           13
 ./bottle-song/src/lib.rs                                 62           58            0            4
 ./minesweeper/src/lib.rs                                 71           56            6            9
 ./space-age/src/lib.rs                                   51           46            0            5
 ./allergies/src/lib.rs                                   56           45            5            6
 ./all-your-base/src/lib.rs                               50           44            0            6
 ./pig-latin/src/lib.rs                                   48           42            0            6
 ./grade-school/src/lib.rs                                42           32            4            6
 ./perfect-numbers/src/lib.rs                             38           32            2            4
 ./dot-dsl/src/graph/graph_items/attrs.rs                 35           29            0            6
 ./sublist/src/lib.rs                                     32           29            0            3
 ./queen-attack/src/lib.rs                                35           28            1            6
 ./dot-dsl/src/graph.rs                                   35           28            0            7
 ./anagram/src/lib.rs                                     32           27            0            5
 ./clock/src/lib.rs                                       32           25            0            7
 ./dot-dsl/src/graph/graph_items/edge.rs                  27           23            0            4
 ./kindergarten-garden/src/lib.rs                         26           23            0            3
 ./matching-brackets/src/lib.rs                           30           23            4            3
 ./high-scores/src/lib.rs                                 28           23            0            5
 ./nth-prime/src/lib.rs                                   28           22            2            4
 ./pascals-triangle/src/lib.rs                            24           21            0            3
 ./dot-dsl/src/graph/graph_items/node.rs                  25           21            0            4
 ./acronym/src/lib.rs                                     28           21            4            3
 ./rna-transcription/src/lib.rs                           25           21            0            4
 ./luhn/src/lib.rs                                        24           20            1            3
 ./bob/src/lib.rs                                         21           19            0            2
 ./prime-factors/src/lib.rs                               20           18            0            2
 ./nucleotide-count/src/lib.rs                            20           17            0            3
 ./proverb/src/lib.rs                                     19           16            0            3
 ./isbn-verifier/src/lib.rs                               17           16            0            1
 ./collatz-conjecture/src/lib.rs                          16           15            1            0
 ./binary-search/src/lib.rs                               37           14           20            3
 ./raindrops/src/lib.rs                                   14           13            0            1
 ./reverse-string/src/lib.rs                              20           13            4            3
 ./sum-of-multiples/src/lib.rs                            11           10            0            1
 ./armstrong-numbers/src/lib.rs                           11           10            0            1
 ./grains/src/lib.rs                                      10            9            0            1
 ./difference-of-squares/src/lib.rs                       17            9            5            3
 ./isogram/src/lib.rs                                     10            8            0            2
 ./hamming/src/lib.rs                                     12            8            2            2
 ./series/src/lib.rs                                       8            8            0            0
 ./etl/src/lib.rs                                          9            7            0            2
 ./eliuds-eggs/src/lib.rs                                  7            6            0            1
 ./leap/src/lib.rs                                        13            6            7            0
 ./gigasecond/src/lib.rs                                   8            5            1            2
 ./pangram/src/lib.rs                                      4            4            0            0
 ./dot-dsl/src/graph/graph_items.rs                        3            3            0            0
 ./hello-world/src/lib.rs                                  4            3            1            0
 ./dot-dsl/src/lib.rs                                      1            1            0            0
====================================================================================================
 Total                                       52         2035         1566          199          270
====================================================================================================
```

> This table was generated with `tokei`. See [this script](https://github.com/eaverdeja/exercism-rust/blob/main/loc-all.sh) for exact params.