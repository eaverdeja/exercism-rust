<!-- omit in toc -->
# Exercism Rust track

Solutions to exercises in the [Exercism Rust track](https://exercism.org/tracks/rust)

This repo and README can be used to search for common language features. I hope you enjoy!

## Disclaimer
> We inherently imitate to learn. None of these solutions are entirely my own - some were
> inspired by community solutions from Exercism, others were ideas proposed by AI.
> There are even a few that were of my own devise, but even then I probably
> tapped into the collective unconscious to come up with them.
> Also, these are just possible solutions to the problems, not the best solutions.
> I come from an object-oriented world and web development, so the code I 
> write is influenced by that.

- [Rust Features by Category](#rust-features-by-category)
  - [Data Structures](#data-structures)
    - [Maps and Sets](#maps-and-sets)
    - [Custom Data Structures](#custom-data-structures)
  - [Less common standard library features](#less-common-standard-library-features)
  - [Iteration Techniques](#iteration-techniques)
    - [Basic Iterator Methods](#basic-iterator-methods)
    - [Advanced Iterator Methods](#advanced-iterator-methods)
    - [Iterator combinators for collection generation](#iterator-combinators-for-collection-generation)
  - [Error Handling](#error-handling)
    - [Result and Option Patterns](#result-and-option-patterns)
  - [Pattern Matching](#pattern-matching)
    - [Match Expressions](#match-expressions)
    - [Enum Patterns](#enum-patterns)
  - [Functional Programming](#functional-programming)
    - [Functional Style](#functional-style)
    - [Immutable Design](#immutable-design)
  - [Bit Manipulation](#bit-manipulation)
    - [Bitwise Operations](#bitwise-operations)
  - [Algorithms](#algorithms)
    - [Search Algorithms](#search-algorithms)
    - [Mathematical Algorithms](#mathematical-algorithms)
  - [String Processing](#string-processing)
    - [String Manipulation](#string-manipulation)
    - [Formatting](#formatting)
  - [Traits and Generics](#traits-and-generics)
    - [Custom Trait Implementations](#custom-trait-implementations)
    - [Generic Programming](#generic-programming)
  - [Memory Management](#memory-management)
    - [Ownership Patterns](#ownership-patterns)
  - [Optimizations](#optimizations)
  - [Design Patterns](#design-patterns)
    - [Builder Pattern](#builder-pattern)
    - [Interpreter Pattern](#interpreter-pattern)
    - [Domain Modeling](#domain-modeling)
  - [Type Conversions and Parsing](#type-conversions-and-parsing)
    - [Number Conversions](#number-conversions)
    - [Single byte conversions](#single-byte-conversions)
    - [Safe Conversions](#safe-conversions)
    - [Custom Type Conversions](#custom-type-conversions)
  - [Tuple and Array Patterns](#tuple-and-array-patterns)
    - [Tuple Operations](#tuple-operations)
    - [Array Techniques](#array-techniques)
- [Solutions to the problems](#solutions-to-the-problems)
  - [Accumulate](#accumulate)
  - [Acronym](#acronym)
  - [Affine Cipher](#affine-cipher)
  - [All-Your-Base](#all-your-base)
  - [Allergies](#allergies)
  - [Alphametics](#alphametics)
  - [Anagram](#anagram)
  - [Armstrong Numbers](#armstrong-numbers)
  - [Atbash Cipher](#atbash-cipher)
  - [Binary Search](#binary-search)
  - [Bob](#bob)
  - [Bottle Song](#bottle-song)
  - [Bowling](#bowling)
  - [Clock](#clock)
  - [Collatz Conjecture](#collatz-conjecture)
  - [Crypto Square](#crypto-square)
  - [Custom Set](#custom-set)
  - [Diamond](#diamond)
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
  - [Largest Series Product](#largest-series-product)
  - [Leap](#leap)
  - [List Ops](#list-ops)
  - [Luhn](#luhn)
  - [Luhn From](#luhn-from)
  - [Luhn Trait](#luhn-trait)
  - [Matching Brackets](#matching-brackets)
  - [Minesweeper](#minesweeper)
  - [Nth Prime](#nth-prime)
  - [Nucleotide Count](#nucleotide-count)
  - [PaaS I/O](#paas-io)
  - [Palindrome Products](#palindrome-products)
  - [Pangram](#pangram)
  - [Pascal's Triangle](#pascals-triangle)
  - [Perfect Numbers](#perfect-numbers)
  - [Phone Number](#phone-number)
  - [Pig Latin](#pig-latin)
  - [Prime Factors](#prime-factors)
  - [Protein Translation](#protein-translation)
  - [Proverb](#proverb)
  - [Queen Attack](#queen-attack)
  - [Rail Fence Cipher](#rail-fence-cipher)
  - [Raindrops](#raindrops)
  - [Reverse String](#reverse-string)
  - [RNA Transcription](#rna-transcription)
  - [Robot Simulator](#robot-simulator)
  - [Robot Name](#robot-name)
  - [Roman Numerals](#roman-numerals)
  - [Rotational Cipher](#rotational-cipher)
  - [Run-length Encoding](#run-length-encoding)
  - [Saddle Points](#saddle-points)
  - [Say](#say)
  - [Scrabble Score](#scrabble-score)
  - [Series](#series)
  - [Sieve of Eratosthenes](#sieve-of-eratosthenes)
  - [Simple Cipher](#simple-cipher)
  - [Simple Linked List](#simple-linked-list)
  - [Space Age](#space-age)
  - [Spiral Matrix](#spiral-matrix)
  - [Sublist](#sublist)
  - [Sum of Multiples](#sum-of-multiples)
  - [Tournament](#tournament)
  - [Triangle](#triangle)
  - [Two Bucket](#two-bucket)
  - [Variable Length Quantity](#variable-length-quantity)
  - [Wordy](#wordy)
- [Testing](#testing)
- [Metrics](#metrics)


# Rust Features by Category

## Data Structures

### Maps and Sets
- `HashMap`/`HashSet` for Efficient Collections - Used in [Allergies](#allergies), [Anagram](#anagram), [ETL](#etl), [Isogram](#isogram), [Nucleotide Count](#nucleotide-count), [Sum of Multiples](#sum-of-multiples), [Robot Name](#robot-name) and many others
- `BTreeMap`/`BTreeSet` for Ordered Collections - Used in [Grade School](#grade-school), [Dot DSL](#dot-dsl)

### Custom Data Structures
- Custom Linked List Implementation - `Option<Box<Node<T>>>` on [Simple Linked List](#simple-linked-list) for safe memory management
- Custom Wrapper Types - Used in [PaaS I/O](#paas-io) for I/O statistics tracking
- Object-oriented Design - Used in [Bowling](#bowling) with multiple structures, [Robot Simulator](#robot-simulator)
- Custom Hash Table - Used in [Custom Set](#custom-set)

## Less common standard library features

- `VecDeque` for queue processing - Used in [Two Bucket](#two-bucket)
- `abs_diff()` for convenient diffing - Used on [Queen Attack](#queen-attack)
- `LazyLock` for lazy static initialization - Used on [Robot Name](#robot-name)

## Iteration Techniques

### Basic Iterator Methods
- `map()`/`filter()`/`collect()` - Used in [Acronym](#acronym), [Armstrong Numbers](#armstrong-numbers), [Atbash Cipher](#atbash-cipher), [Scrabble Score](#scrabble-score), [Difference of Squares](#difference-of-squares) and the list goes on! These ones are hard *not* to find in a solution! 
- `windows()` - Used in [Largest Series Product](#largest-series-product), [Proverb](#proverb), [Series](#series), [Sublist](#sublist) for processing adjacent elements
- `chunks()` - Used in [Protein Translation](#protein-translation)
- `enumerate()` - Used in [Saddle Points](#saddle-points), [All-Your-Base](#all-your-base), [ISBN Verifier](#isbn-verifier), and many others. This one is also pretty popular!
- `zip()` - Used in [Hamming](#hamming), [Simple Cipher](#simple-cipher)
- `flat_map()` - Used in [Alphametics](#alphametics), [ETL](#etl), [Kindergarten Garden](#kindergarten-garden), [Sum of Multiples](#sum-of-multiples), [Variable Length Quantity](#variable-length-quantity)
- `fold()` - Used in [Luhn](#luhn), [Robot Simulator](#robot-simulator), [Pascal's Triangle](#pascals-triangle), [Rail Fence Cipher](#rail-fence-cipher)
- `chain()` - Used in [Bottle Song](#bottle-song), [Robot Name](#robot-name)
- `nth()` - Used in [Alphametics](#alphametics), [Phone Number](#phone-number), [Nth Prime](#nth-prime)
- `flatten()`- Used in [Custom Set](#custom-set), [Rail Fence Cipher](#rail-fence-cipher)
- `cycle()` - Used in [Simple Cipher](#simple-cipher)

### Advanced Iterator Methods
- `try_fold()` - Used in [All-Your-Base](#all-your-base), [ISBN Verifier](#isbn-verifier), [RNA Transcription](#rna-transcription)
- `peekable()` - Used in [Pig Latin](#pig-latin), [Run-length Encoding](#run-length-encoding), [Wordy](#wordy)
- `step_by()` - Used in [Sieve of Eratosthenes](#sieve-of-eratosthenes), [Sum of Multiples](#sum-of-multiples)
- `iter::once()` - Used in [Acronym](#acronym), [Alphametics](#alphametics), [Nth Prime](#nth-prime), [Proverb](#proverb)
- `itertools::fold_while()` - Used in [Protein Translation](#protein-translation)
- `iter::from_fn()` for creating custom stateful iterators - Used in [List Ops](#list-ops)

### Iterator combinators for collection generation

- Functional string building with `map()`, `filter()`, `chain()`, `collect()` patterns - Used in [Atbash Cipher](#atbash-cipher), [Crypto Square](#crypto-square), [Robot Name](#robot-name)
- Sequence generation - Used in [Palindrome Products](#palindrome-products) for generating factor pairs
- String transformation - Used in [Reverse String](#reverse-string) with `chars().rev().collect()` pattern
- Subslice extraction - Used in [Series](#series) with `windows()` combined with `map()` and `collect()`
- Filtered collection building - Used in [Alphametics](#alphametics) and [Sieve of Eratosthenes](#sieve-of-eratosthenes) with `filter_map()`
- Incremental construction with `fold()` - Used in [Pascal's Triangle](#pascals-triangle) for building rows from previous ones

## Error Handling

### Result and Option Patterns
- Custom `Error` Enums - Used in [All-Your-Base](#all-your-base), [Bowling](#bowling), [Variable Length Quantity](#variable-length-quantity) and others
- `Option` for Missing Values & Error Handling - Used in [Binary Search](#binary-search), [High Scores](#high-scores), [Two Bucket](#two-bucket), [Protein Translation](#protein-translation), [Wordy](#wordy), [Rail Fence Cipher](#rail-fence-cipher)
- `?` Operator - Used in [Perfect Numbers](#perfect-numbers), [Sieve of Eratosthenes](#sieve-of-eratosthenes), [Variable Length Quantity](#variable-length-quantity), [Wordy](#wordy)
- `ok_or()` for converting `Option` to `Result` - Used in [Largest Series Product](#largest-series-product)
- `or_else()` for dynamically handling `None` values - Used in [List Ops](#list-ops)

## Pattern Matching

### Match Expressions
- Pattern Matching with Guards - Used in [Bob](#bob), [Leap](#leap)
- Complex String Parsing - Used in [Pig Latin](#pig-latin)
- Multi-pattern Matching with `|` operator - Used in [Scrabble Score](#scrabble-score), [Protein Translation](#protein-translation), [Wordy](#wordy)
- Matching against byte string literals - Used in [Protein Translation](#protein-translation)
- `@` binding operator - Used in [Wordy](#wordy)

### Enum Patterns
- Explicit Discriminators - Used in [Allergies](#allergies)
- Result Classification - Used in [Sublist](#sublist), [Perfect Numbers](#perfect-numbers)
- Branching - Used in [Simple Cipher](#simple-cipher)

## Functional Programming

### Functional Style
- Method Chaining - Used in [Raindrops](#raindrops), [Acronym](#acronym)
- Closures for Operations - Used in [Acronym](#acronym), [Spiral Matrix](#spiral-matrix)
- Higher-order functions with closure parameters (`FnMut`) - Used in [Accumulate](#accumulate)

### Immutable Design
- Pure Functions - Used in [Clock](#clock) returning new objects
- Immutable State Transformations - Used in [Two Bucket](#two-bucket) via `with()` method, [Robot Simulator](#robot-simulator) via `with_*` methods
- Spread operator to keep unchanged fields when creating new objects (`..self`) - Used in [Robot Simulator](#robot-simulator)

## Bit Manipulation

### Bitwise Operations
- Flags and Bit Testing - Used in [Allergies](#allergies) with bitwise `&`
- Bit Counting - Used in [Eliud's Eggs](#eliuds-eggs) with shift and mask
- Byte-by-byte decoding - Used in [Variable Length Quantity](#variable-length-quantity)

## Algorithms

### Search Algorithms
- Binary Search - Used in [Binary Search](#binary-search)
- Breadth-first Search - Used in [Two Bucket](#two-bucket) with `VecDeque`
- Backtracking - Used in [Alphametics](#alphametics)

### Mathematical Algorithms
- Sieve of Eratosthenes - Used in [Sieve of Eratosthenes](#sieve-of-eratosthenes)
- Prime Checking - Used in [Nth Prime](#nth-prime) with square root optimization
- Factor Finding - Used in [Perfect Numbers](#perfect-numbers) with square root optimization, [Palindrome Products](#palindrome-products) to find palindrome products
- Modular arithmetic with `rem_euclid()` - Used in [Affine Cipher](#affine-cipher), [Clock](#clock)
- Greatest common divisor (GCD) calculation - Used in [Affine Cipher](#affine-cipher), [Two Bucket](#two-bucket)
- Positional digit extraction with modulo and division operations - Used in [Roman Numerals](#roman-numerals)

## String Processing

### String Manipulation
- Simple sanitization & validation - Used in [Phone Number](#phone-number)
- Unicode-aware Processing - Used in [Anagram](#anagram), [Reverse String](#reverse-string)
- Number-to-Words Conversion - Used in [Say](#say) with recursive approach
- Run-length Encoding - Used in [Run-length Encoding](#run-length-encoding)
- `chunks()` on `&[char]` - Used in [Affine Cipher](#affine-cipher), [Atbash Cipher](#atbash-cipher), [Protein Translation](#protein-translation)
- Byte-level string manipulation with `as_bytes()` - Used in [Largest Series Product](#largest-series-product), [Minesweeper](#minesweeper), [Protein Translation](#protein-translation)
- Substring extraction with `strip_prefix()` and `strip_suffix()` - Used in [Wordy](#wordy)
- Grid-based text transpostion - Used in [Crypto Square](#crypto-square)
- Character-by-character processing with positional tracking - Used in [Rail Fence Cipher](#rail-fence-cipher)

### Formatting
- String Formatting - Used in [Clock](#clock), [Tournament](#tournament) with format specifiers
- String Building with `format!()` macro - Used in [Bottle Song](#bottle-song), [Diamond](#diamond), [Roman Numerals](#roman-numerals)

## Traits and Generics

### Custom Trait Implementations
- `Display` and `PartialEq` - Used in [Clock](#clock)
- `From` and Type Conversion - Used in [Space Age](#space-age), [Roman Numerals](#roman-numerals)

### Generic Programming
- Generic Functions - Used in [Binary Search](#binary-search) with type parameters, [Simple Linked List](#simple-linked-list) with `Node<T>`, [Accumulate](#accumulate), [List Ops](#list-ops)
- Generic `impl` with Trait Bounds - Used in [Custom Set](#custom-set), [Luhn From](#luhn-from), [Simple Linked List](#simple-linked-list), [Triangle](#triangle)
- Generic `trait` - Used in [Luhn Trait](#luhn-trait)


## Memory Management

### Ownership Patterns
- Borrowing with Lifetimes - Used in [High Scores](#high-scores), [Anagram](#anagram)
- Zero-copy Parsing - Used in [Tournament](#tournament) with string borrows
- `Box<T>` for Heap Allocation - Used in [Simple Linked List](#simple-linked-list)
- Slices and slice indexing - Used in [Variable Length Quantity](#variable-length-quantity)
- `Mutex` for safe shared state management - Used in [Robot Name](#robot-name)
- `move` closures to transfer ownership - Used in [List Ops](#list-ops)

## Optimizations
- Pre-allocating space with `Vec::with_capacity()` - Used in [Custom Set](#custom-set), [PaaS I/O](#paas-io), [Variable Length Quantity](#variable-length-quantity)

## Design Patterns

### Builder Pattern
- Fluent Interfaces - Used in [Dot DSL](#dot-dsl)
- Method Chaining - Used in [Tournament](#tournament) with `entry` API

### Interpreter Pattern
- Tokenization and Evaluation - Used in [Wordy](#wordy)

### Domain Modeling
- Type-safe Domain Models - Used in [Tournament](#tournament), [Robot Simulator](#robot-simulator)
- State Encapsulation - Used in [Bowling](#bowling)
- Abstraction with Enums - Used in [Spiral Matrix](#spiral-matrix), [Robot Simulator](#robot-simulator)

## Type Conversions and Parsing

### Number Conversions
- `to_digit(1)` - Used in [Luhn](#luhn), [Armstrong Numbers](#armstrong-numbers) for char-to-digit conversion
- `parse()` - Used in [Run-length Encoding](#run-length-encoding), [Say](#say) for string-to-number parsing
- `as_bytes()` - Used in [Minesweeper](#minesweeper) for grid operations

### Single byte conversions
- ASCII arithmetic - Used in [Atbash Cipher](#atbash-cipher), [Diamond](#diamond), [Largest Series Product](#largest-series-product), [Rotational Cipher](#rotational-cipher), [Simple Cipher](#simple-cipher)

### Safe Conversions
- `checked_*` Operations - Used in [Collatz Conjecture](#collatz-conjecture) to prevent arithmetic overflow
- String/Integer Conversion - Used in [Armstrong Numbers](#armstrong-numbers), [ISBN Verifier](#isbn-verifier)
- `TryFrom`/`TryInto` - Used in [All-Your-Base](#all-your-base) for fallible conversions

### Custom Type Conversions
- Custom `From` Implementations - Used in [Luhn From](#luhn-from), [Space Age](#space-age)
- Manual Parsing Logic - Used in [Nucleotide Count](#nucleotide-count), [Acronym](#acronym) for domain-specific parsing
- Type-driven Design - Used in [Tournament](#tournament) with newtypes to prevent mixing of related values

## Tuple and Array Patterns

### Tuple Operations
- Tuple Destructuring - Used in [Sublist](#sublist), [Queen Attack](#queen-attack) to unpack related values 
- Tuple-based Returns - Used in [Palindrome Products](#palindrome-products) for factor pairs
- Tuple Pattern Matching - Used in [Hamming](#hamming) with `zip().filter(|(a, b)| a != b)` pattern

### Array Techniques
- Fixed-size Arrays - Used in [Raindrops](#raindrops) with array of tuples
- Array Destructuring - Used in [Triangle](#triangle) for brevity and pattern matching
- Array of Constants - Used in [Say](#say), [Space Age](#space-age) for lookup tables
- Static array for lookup tables - Used in [Roman Numerals](#roman-numerals)

# Solutions to the problems

Below is a brief analysis of each solution. Solutions are ordered alphabetically.

## [Accumulate](https://github.com/eaverdeja/exercism-rust/tree/main/accumulate/src/lib.rs)

- Generic function with `FnMut(T) -> U` signature for closure parameter
- Functional programming with higher-order functions
- Creates new collection rather than mutating in place
- Manual collection transformation without standard library iterators  

## [Acronym](https://github.com/eaverdeja/exercism-rust/tree/main/acronym/src/lib.rs)

- Method chaining with `split`, `filter`, `map`, `collect`
- Closures for filtering and mapping operations
- Iterator combinators (`once`, `chain`, `skip`)
- Separate function handling special cases
- Pattern matching with `char` predicates

## [Affine Cipher](https://github.com/eaverdeja/exercism-rust/tree/main/affine-cipher/src/lib.rs)

- Modular arithmetic operations for encryption/decryption with `rem_euclid()`
- Extended Euclidean algorithm for calculating modular multiplicative inverse (MMI)
- Functional approach with chain of iterator methods (`filter()` `map()`, multiple `collect()`s with turbofish)
- String processing with chunking for formatted output with `chunks()` and `join()`
- Mathematical validation of key properties (coprime check)

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

## [Atbash Cipher](https://github.com/eaverdeja/exercism-rust/tree/main/atbash-cipher/src/lib.rs)

- Direct character encoding/decoding without lookup tables
- Leverages ASCII arithmetic for character substitution with `b'z' - (char - b'a')` pattern
- Functional approach with chained methods like `map()`, `collect()` and `enumerate()`
- String chunking with `chunks()` and `join(" ")` for encoding output
- Character manipulation with `is_ascii_alphanumeric()` and `to_ascii_lowercase()`

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

## [Crypto Square](https://github.com/eaverdeja/exercism-rust/tree/main/crypto-square/src/lib.rs)

- String normalization with `filter()` and `to_ascii_lowercase()`
- Uses `div_ceil()` integer division with ceiling for rectangle sizing
- Optimal rectangle dimensions found through sequence generation with `(1..).map().find()`
- Handling of incomplete rectangles with `unwrap_or(&' ')` for padding
- Functional string building with nested `map()` operations and `collect()`
- Joins output columns with spaces using `collect::<Vec<_>>().join(" ")`

## [Custom Set](https://github.com/eaverdeja/exercism-rust/tree/main/custom-set/src/lib.rs)

- Custom hash table implementation with linear probing for collision resolution
- Implements standard set operations: `union`, `intersection`, `difference`
- Comparison operations like `is_subset`, `is_disjoint`
- Maintains O(1) average-case complexity for basic operations
- Manual memory management with `Vec::with_capacity`
- Dynamic resizing to maintain performance characteristics
- Provides simple iterator access via custom `iter()` method
- Uses `Hash` trait for key hashing with `DefaultHasher`
- Implements `PartialEq`, `Eq`, `FromIterator` and `Clone` traits
- Uses generics with trait bounds (`Hash + Eq + Clone`)

## [Diamond](https://github.com/eaverdeja/exercism-rust/tree/main/diamond/src/lib.rs)

- Character manipulation with offsets (`as u8` and `as char`)
- Handles whitespace padding based on calculated dimensions with `format!` and width/fill modifiers (`width$`, `^`, `<`, `>`)

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

## [Largest Series Product](https://github.com/eaverdeja/exercism-rust/tree/main/largest-series-product)

- `windows()` method for sliding window operations over bytes with `as_bytes()`
- Direct byte manipulation with ASCII arithmetic (`byte - b'0'`)
- Uses `ok_or()` to convert `Option` to `Result` for the empty string case

## [Leap](https://github.com/eaverdeja/exercism-rust/tree/main/leap/src/lib.rs)

- Pattern matching for different cases
- Readable approach to complex boolean logic
- Performance-optimized conditionals

## [List Ops](https://github.com/eaverdeja/exercism-rust/tree/main/list-ops)

- Implements fundamental list operations with custom iterator functions
- Uses `iter::from_fn` to create stateful iterators without explicit structs
- Handles generic types with traits (`Iterator`, `DoubleEndedIterator`)
- Uses `next_back` to process iterators in reverse
- Uses closure state capture for stateful iteration with `move`

## [Luhn](https://github.com/eaverdeja/exercism-rust/tree/main/luhn/src/lib.rs)

- Input validation with multiple conditions
- Advanced iterator operations
- Numeric conversion with `to_digit(10)`
- Modulo checking for algorithm validation

## [Luhn From](https://github.com/eaverdeja/exercism-rust/tree/main/luhn-from/src/lib.rs)

- Uses generics and trait bounds with `impl<T: Display> From<T> for Luhn` to allow for the construction of custom `Lunh` structs from any input type that is displayable

## [Luhn Trait](https://github.com/eaverdeja/exercism-rust/tree/main/luhn-trait/src/lib.rs)

- Avoids the use of a custom struct entirely by defining a generic `Luhn<T>` trait and implementing it for displayable types with `impl<T: Display> Luhn<T> for T`



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

## [Phone Number](https://github.com/eaverdeja/exercism-rust/tree/main/phone-number)

- String filtering with `filter(char::is_ascii_digit)` to extract just numeric characters
- Control flow with early returns and `?` operator for propagating `None` values
- `strip_prefix()` to conditionally handle country code "1"
- Range patterns with `('2'..='9').contains(&char)` for NANP validation rules
- `chars().nth()` to check specific positions in the normalized number

## [Pig Latin](https://github.com/eaverdeja/exercism-rust/tree/main/pig-latin/src/lib.rs)

- Complex string parsing with pattern matching
- `split_at()` for string manipulation
- Helper functions for concern separation
- `peekable()` iterator for lookahead

## [Prime Factors](https://github.com/eaverdeja/exercism-rust/tree/main/prime-factors/src/lib.rs)

- Loop with pattern matching
- Trial division algorithm
- Efficient value/divisor updates

## [Protein Translation](https://github.com/eaverdeja/exercism-rust/tree/main/protein-translation/src/lib.rs)

- Leverages `chunks(3)` for processing 3-character codons
- Uses pattern matching with multi-pattern branches for codon translation
- Matches against byte string literals (`b"UAA"`) to allow matching on a slice of bytes (`&[u8]`)
- Uses `itertools::fold_while` with `Continue` and `Done` for more fine-grained folding with early exit capability
- Returns `Option<Vec<&str>>` to handle both valid translations and errors

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

## [Rail Fence Cipher](https://github.com/eaverdeja/exercism-rust/tree/main/rail-fence-cipher)

- Uses `fold()` with a vector of vectors as accumulator to organize characters by rail
- Encoding creates the fence as a `Vec<Vec<char>>` and `flatten()`s it to create the encoded result
- Decoding approach uses a two-phase strategy:
  1. First creates a template with position markers
  2. Then fills the template with characters from the cipher text in rail order
- Uses `filter_map()` to extract valid positions from the template
- Creates an initial placeholder vector with `vec!['?'; cipher_len]` that's filled during decoding

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

## [RNA Transcription](https://github.com/eaverdeja/exercism-rust/tree/main/rna-transcription/src/lib.rs)

- Tuple structs with private data (`Dna(String)`, `Rna(String)`)
- Pattern matching with match for nucleotide conversion
- `try_fold` for validation with early return on error

## [Robot Simulator](https://github.com/eaverdeja/exercism-rust/tree/main/robot-simulator/src/lib.rs)

- Pattern matching with `match` expressions for direction changes
- Functional programming with `fold()` and method chaining to process instruction sequences
- Immutable state transitions (`with_direction`, `with_coordinates`) for clean state updates
- Domain modeling with custom `Enum` for robot directions

## [Robot Name](https://github.com/eaverdeja/exercism-rust/tree/main/robot-name/src/lib.rs)

- Uses `LazyLock<Mutex<HashSet<String>>>` for global state management to track used robot names
  - Note that this is generally discouraged, but was implemented for the sake of the exercise. Still a good to know!
- Implements thread-safe name uniqueness checking with synchronization primitives (`lock()`)
- Leverages the `rand` crate for random character generation
- Uses iterator combinators (`map()`, `chain()`, `collect()`) for functional name construction

## [Roman Numerals](https://github.com/eaverdeja/exercism-rust/tree/main/roman-numerals/src/lib.rs)

- Static lookup tables (`&[&str]`) for digit-to-numeral conversion
- Implementation of `Display` trait for formatting
- Custom `From<u32>` implementation for type conversion
- Positional digit extraction with modulo and division operations
- String composition with `format!` macro

## [Rotational Cipher](https://github.com/eaverdeja/exercism-rust/tree/main/rotational-cipher)

- Uses character range patterns (`'a'..='z'`, `'A'..='Z'`) for matching letter cases
- Character manipulation with ASCII arithmetic (`char` to `u8` and back)
- Implements Caesar cipher with modulo operation to handle wrapping (`% 26`)
- Maintains non-alphabetic characters unchanged with wildcard pattern matching

## [Run-length Encoding](https://github.com/eaverdeja/exercism-rust/tree/main/run-length-encoding/src/lib.rs)

- `peekable()` iterator for lookahead while encoding
- Iterative approach with state tracking variables for decoding
- `parse()` with error handling for safe number conversion
- `repeat()` method for character replication during decoding

## [Saddle Points](https://github.com/eaverdeja/exercism-rust/tree/main/saddle-points/src/lib.rs)

- `iter().enumerate()` with tuple unpacking to track indices
- Precomputes column minimums for efficiency
- Combines row maxima and column minima checks by using `map()`, `min()`, `max()` methods

## [Say](https://github.com/eaverdeja/exercism-rust/tree/main/say/src/lib.rs)

- Module organization with separate constants module
- Arrays of string literals (`&'static str`) for number names and scale words
- Divide-and-conquer approach with recursive number processing
- Breaking numbers into three-digit chunks with modulo and division operations
- String formatting with the `format!` macro for constructing output
- Pattern matching on number ranges (small numbers vs. large numbers)

## [Scrabble Score](https://github.com/eaverdeja/exercism-rust/tree/main/scrabble-score/src/lib.rs)

- Leverages functional programming with `map()` and `sum()` for concise implementation
- Extensive pattern matching with the match expression for letter scoring
- Pattern grouping with the `|` operator
- Uses `to_ascii_uppercase()` to handle case-insensitive scoring
- Uses a fallback with `_ => 0` to handle non-scoring characters gracefully

## [Series](https://github.com/eaverdeja/exercism-rust/tree/main/series/src/lib.rs)

- `windows()` for sliding window operations
- Efficient collection transformation
- Concise solution with chained methods

## [Sieve of Eratosthenes](https://github.com/eaverdeja/exercism-rust/tree/main/sieve/src/lib.rs)

- Classic Sieve of Eratosthenes implementation with vector of optional numbers
- Control flow with the `?` operator, `take()` and `filter_map()`
- Performance optimization with `i.pow(2)` to start marking from square of prime
- `step_by()` to efficiently iterate through multiples
- Type conversion with as `usize` for indexing

## [Simple Cipher](https://github.com/eaverdeja/exercism-rust/tree/main/simple-cipher)

- Uses `enum Op` for cryptographic operation selection (encode/decode)
- Character manipulation with ASCII arithmetic and range patterns (`'a'..='z'`, `'A'..='Z'`)
- `zip()` with `cycle()` to pair plaintext with repeating key characters 
- Uses the `rand` crate with `SampleString` and `Uniform` distribution for key generation

## [Simple Linked List](https://github.com/eaverdeja/exercism-rust/tree/main/simple-linked-list/src/lib.rs)

- Implements standard singly linked list operations (`new`, `push`, `pop`, `peek`, `rev`)
- `Option<Box<Node<T>>>` for safe memory management
- Generic implementation with type parameter `T`
- Handling of ownership with methods like `take()` and `as_ref()`
- `rev()` reuses nodes rather than creating new ones
- Implements `FromIterator` for collection conversion and provides `into` for `Vec` conversion
- Time complexity maintained appropriately: `O(1)` for `push/pop`, `O(n)` for traversals

## [Space Age](https://github.com/eaverdeja/exercism-rust/tree/main/space-age/src/lib.rs)

- `const` values for orbital periods
- `trait` with default implementation
- Empty structs implementing shared trait
- `From` trait for type conversion
- Single calculation applying to all planets

## [Spiral Matrix](https://github.com/eaverdeja/exercism-rust/tree/main/spiral-matrix/src/lib.rs)

- Models the problem using `Direction` and `Cursors` abstractions for clear semantics
- `O(n²)` time complexity with a single pass through the data
- Uses functional style with iterators and closures
- Leverages pattern matching and block expressions for control flow
- Separates motion logic from value placement logic

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

## [Tournament](https://github.com/eaverdeja/exercism-rust/tree/main/tournament/src/lib.rs)

- Type-safe domain models using structs
- Memory efficiency and zero-copy parsing by borrowing strings with lifetimes
- Newtype pattern and `HashMap` delegation with `Deref/DerefMut`
- Builder pattern and method chaining with `entry` API
- Optimized sorting with `sort_by_cached_key`
- String formatting logic with format specifiers

## [Triangle](https://github.com/eaverdeja/exercism-rust/tree/main/triangle/src/lib.rs)

- Use of trait bounds and generics to allow both integer and float triangles
- Array destructuring for brevity

## [Two Bucket](https://github.com/eaverdeja/exercism-rust/tree/main/two-bucket/src/lib.rs)

- Uses breadth-first search (BFS) to find the minimum number of moves
- Uses `VecDeque` to process states with `push_back()` and `pop_front()`
- Tracks visited states with a `HashSet` to avoid cycles
- `State` struct encapsulates bucket state and operations
- Uses `with()` method for creating new states immutably
- Validates solvability using GCD property before attempting search
- Separates state manipulation logic from search algorithm
- Helper functions isolate mathematical logic

## [Variable Length Quantity](https://github.com/eaverdeja/exercism-rust/tree/main/variable-length-quantity/src/lib.rs)

- Iterator combinators like `flat_map()`, `collect()`, and `map()`
- Leverages Rust's pattern of returning `Result<T, E>` for error handling
- Uses the `?` operator for concise error propagation
- Employs slices (`&[u8]`, `&[u32]`) to work with data without taking ownership
- Returns a tuple `(value, bytes_read)` for tracking parser position without maintaining state
- Uses `Vec::with_capacity` to optimize memory allocation when the size is known
- Takes advantage of bitwise operations (`<<`, `>>`, `&`, `|`) for efficient bit manipulation
- Implements the visitor pattern via slice indexing (`&bytes[i..]`) to advance through data
- Combines functional and imperative styles

## [Wordy](https://github.com/eaverdeja/exercism-rust/tree/main/wordy/src/lib.rs)

- Two-phase interpreter design (tokenize, evaluate)
- Immutable approach with passing state between phases
- `Token` enum with variants for numbers and operations
- Stateful tokenizer using `split_whitespace().peekable()` for lookahead parsing via `next_if_eq()`
- Pattern matching with destructuring, match guards and the `@` binding operator in match arms
- Pipeline architecture with `Option` and the `?` operator for clean error propagation
- `Option` handling with `is_some_and()` and `take()`
- String manipulation with `strip_prefix()` and `strip_suffix()`

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

# Testing

This repo is structured as a Cargo workspace, in where each solution is a "member". This allows us to run tests for all solutions in one go with:
```console
$ cargo test --workspace --test '*'
```

This is taking ~2.7s on my M1 max 32GB.
The main offenders here are [Alphametics](#alphametics) and [Palindrome Products](#palindrome-products). Alphametics is especially brutal with its [final boss test case](https://github.com/eaverdeja/exercism-rust/blob/main/alphametics/tests/alphametics.rs#L105).


# Metrics

Here are all solutions ordered by lines of code. Use this a proxy for how complex or elegant a solution is 😄

> This breakdown is not perfect since solutions with multiple files
> are not grouped together. But alas!

```console
====================================================================================================
 Language                                 Files        Lines         Code     Comments       Blanks
====================================================================================================
 Rust                                        82         3802         3004          267          531
----------------------------------------------------------------------------------------------------
 ./alphametics/src/lib.rs                                463          289          111           63
 ./bowling/src/lib.rs                                    211          161           17           33
 ./custom-set/src/lib.rs                                 193          140           21           32
 ./tournament/src/lib.rs                                 150          126            0           24
 ./two-bucket/src/lib.rs                                 159          123           15           21
 ./spiral-matrix/src/lib.rs                               93           81            0           12
 ./robot-simulator/src/lib.rs                             94           80            0           14
 ./wordy/src/lib.rs                                       94           78            5           11
 ./simple-linked-list/src/lib.rs                         108           77           10           21
 ./palindrome-products/src/lib.rs                         86           73            1           12
 ./affine-cipher/src/lib.rs                               87           70            3           14
 ./paasio/src/lib.rs                                      79           66            0           13
 ./list-ops/src/lib.rs                                    74           66            0            8
 ./rail-fence-cipher/src/lib.rs                           75           59            4           12
 ./variable-length-quantity/src/lib.rs                    75           59            3           13
 ./bottle-song/src/lib.rs                                 62           58            0            4
 ./minesweeper/src/lib.rs                                 71           56            6            9
 ./say/src/lib.rs                                         72           51            5           16
 ./robot-name/src/lib.rs                                  58           46            0           12
 ./space-age/src/lib.rs                                   51           46            0            5
 ./allergies/src/lib.rs                                   56           45            5            6
 ./all-your-base/src/lib.rs                               50           44            0            6
 ./pig-latin/src/lib.rs                                   48           42            0            6
 ./rna-transcription/src/lib.rs                           47           41            0            6
 ./run-length-encoding/src/lib.rs                         42           36            0            6
 ./grade-school/src/lib.rs                                42           32            4            6
 ./perfect-numbers/src/lib.rs                             38           32            2            4
 ./luhn-trait/src/lib.rs                                  35           30            0            5
 ./luhn-from/src/lib.rs                                   35           29            0            6
 ./dot-dsl/src/graph/graph_items/attrs.rs                 35           29            0            6
 ./sublist/src/lib.rs                                     32           29            0            3
 ./dot-dsl/src/graph.rs                                   35           28            0            7
 ./queen-attack/src/lib.rs                                35           28            1            6
 ./roman-numerals/src/lib.rs                              32           28            0            4
 ./anagram/src/lib.rs                                     32           27            0            5
 ./protein-translation/src/lib.rs                         29           27            0            2
 ./clock/src/lib.rs                                       32           25            0            7
 ./atbash-cipher/src/lib.rs                               27           25            0            2
 ./triangle/src/lib.rs                                    31           25            0            6
 ./dot-dsl/src/graph/graph_items/edge.rs                  27           23            0            4
 ./kindergarten-garden/src/lib.rs                         26           23            0            3
 ./matching-brackets/src/lib.rs                           30           23            4            3
 ./high-scores/src/lib.rs                                 28           23            0            5
 ./nth-prime/src/lib.rs                                   28           22            2            4
 ./crypto-square/src/lib.rs                               25           22            0            3
 ./largest-series-product/src/lib.rs                      25           22            0            3
 ./dot-dsl/src/graph/graph_items/node.rs                  25           21            0            4
 ./acronym/src/lib.rs                                     28           21            4            3
 ./pascals-triangle/src/lib.rs                            24           21            0            3
 ./saddle-points/src/lib.rs                               23           20            0            3
 ./luhn/src/lib.rs                                        24           20            1            3
 ./bob/src/lib.rs                                         21           19            0            2
 ./diamond/src/lib.rs                                     20           19            0            1
 ./prime-factors/src/lib.rs                               20           18            0            2
 ./phone-number/src/lib.rs                                20           18            0            2
 ./nucleotide-count/src/lib.rs                            20           17            0            3
 ./proverb/src/lib.rs                                     19           16            0            3
 ./isbn-verifier/src/lib.rs                               17           16            0            1
 ./collatz-conjecture/src/lib.rs                          16           15            1            0
 ./rotational-cipher/src/lib.rs                           16           15            0            1
 ./binary-search/src/lib.rs                               37           14           20            3
 ./scrabble-score/src/lib.rs                              14           14            0            0
 ./raindrops/src/lib.rs                                   14           13            0            1
 ./reverse-string/src/lib.rs                              20           13            4            3
 ./sieve/src/lib.rs                                       17           12            2            3
 ./accumulate/src/lib.rs                                  10           10            0            0
 ./say/src/constants.rs                                   12           10            0            2
 ./armstrong-numbers/src/lib.rs                           11           10            0            1
 ./sum-of-multiples/src/lib.rs                            11           10            0            1
 ./grains/src/lib.rs                                      10            9            0            1
 ./difference-of-squares/src/lib.rs                       17            9            5            3
 ./hamming/src/lib.rs                                     12            8            2            2
 ./isogram/src/lib.rs                                     10            8            0            2
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
 Total                                       82         3802         3004          267          531
====================================================================================================
```

> This table was generated with `tokei`. See [this script](https://github.com/eaverdeja/exercism-rust/blob/19d4f8b4bc6235769322e6cc57be4e463cd32e8e/justfile#L6) for exact params.