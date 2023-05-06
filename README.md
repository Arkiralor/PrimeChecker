# Prime-Checker

A sample rust package to check the prime-ness of a given unsigned, 64-bit integer.

## Description

_Do note that wherever we use square brackets in this section, we are using the mathematical expression for an inclusive boundary._

Any whole number (or natural number, depending on who you ask) `num`, can have one of the following three prime-ness values:

1. __Prime:__ The list of its factors is exactly `[1, num]`.
2. __Composite:__ The list of its factors is `[1, [z (- Z], num]` where `z` can be any natural number exclusively between `1` and `num`.
3. __Anti-Prime:__ The list of its factors is the same as a composite number _(except in special cases, such as `1`, `2`)_ but the length of the list is the greatest for the set [`num-z`, `num`] _i.e, it exclusively has the highest number of factors for any natural number less than it_.

The library here holds functions that help determine and select unsigned, 64-bit integers depending on these three criteria.

## Documentation

As this entire package is a cargo crate, you can generate the documentation using `cargo doc` to get specific details, as well. Here we will go over the very basics of the functions defined in the `lib.rs` file.

1. `check_if_anti_prime()`
  
    - _Checks if the given number is an anti-prime number._
    - __Arguments:__ `num: u64`
    - __Returns:__ `bool | default: false`, `Vec<u64>` where the `vector` is list of the given number, `num`'s factors.
    - __Usage:__

        ```rs
        use prime_checker;

        fn main(){
            let num: u64 = z; // z belongs to the set of natural numbers and is only used as a placeholder by us in this README.
            let check: bool;
            let factors: Vec<u64>;

            (check, factors) = prime_checker::check_if_anti_prime(num: num);
            if check == true{
                println!("{} is an anti-prime number.", num);
            }
            else {
                println!("{} is not an anti-prime number; here are its factors: {:?}", num, factors);
            }
        }
        ```

2. `check_if_prime()`

    - _Checks if the given number is a prime number._
    - __Arguments:__ `num: u64`
    - __Returns:__ `bool | default: false`, `Vec<u64>` where the `vector` is list of the given number, `num`'s factors.
    - __Usage:__

        ```rs
        use prime_checker;

        fn main(){
            let num: u64 = z; // z belongs to the set of natural numbers and is only used as a placeholder by us in this README.
            let check: bool;
            let factors: Vec<u64>;

            (check, factors) = prime_checker::check_if_prime(num: num);
            if check == true{
                println!("{} is a prime number.", num);
            }
            else {
                println!("{} is not a prime number; here are its factors: {:?}", num, factors);
            }
        }
        ```

3. `description()`

    - _Returns a brief description of this library crate and if `show` is set to `true`, prints the same to the console._
    - __Arguments:__ `show: bool`
    - __Returns:__ `String`
    - __Usage:__

        ```rs
        use prime_checker;

        fn main(){
            let desc_str = prime_checker::description(true);
        }
        ```

4. `find_primes_till()`

    - _Finds all prime numbers which are less than or equal to the given number._
    - __Arguments:__ `num: u64`
    - __Returns:__ `Vec<u64>` where the `vector` is list all prime numbers which are less than or equal to the given number, `num`.
    - __Usage:__

        ```rs
        use prime_checker;

        fn main(){
            let num: u64 = z; // z belongs to the set of natural numbers and is only used as a placeholder by us in this README.
            let prime_numbers: Vec<u64>;

            prime_numbers = prime_checker::find_primes_till(num: num);
            println!("The prime numbers till {} are:\t{:?}", num, prime_numbers);
        }
        ```

5. `find_anti_primes_till()`

    - _Finds all anti-prime numbers which are less than or equal to the given number._
    - __Arguments:__ `num: u64`
    - __Returns:__ `Vec<u64>` which is the list of all anti-prime numbers less than or equal to the given number, `num`.
    - __Usage:__

        ```rs
        use prime_checker;

            fn main(){
                let num: u64 = z; // z belongs to the set of natural numbers and is only used as a placeholder by us in this README.
                let anti_prime_numbers: Vec<u64>;

                anti_prime_numbers = prime_checker::find_anti_primes_till(num: num);
                println!("The anti-prime numbers till {} are:\t{:?}", num, anti_prime_numbers);
            }
        ```

    - __WARNING:__ _HIGHLY Unoptimized and Computationally Expensive_[^1].

## Development and Contribution

If you want to contribute to this library, kindly follow the steps described below.

### Contribution Workflow

1. Assign or ask a moderator to assign yourself to the required issue; this is to ensure that the same issue is not being independently resolved by two or more unrelated parties.
2. If not already done, fork the repository to your own github account.
3. If this is your first time contributing, follow the steps given in the following section to set up the development environment.
4. Checkout a branch named as an url-safe version of the issue number.
    - Suppose the issue number is `Misc_001`, then the branch name will be `misc-001`.
5. Make the code changes as required.
   - Also, write any additional test-cases required in the required module/file.
6. Run `cargo test --verbose` to make sure everything works and is validated.
   - Do __NOT__ proceed further until all tests pass.
7. Commit your changes with a meaningful commit message.
8. Merge with the `master` branch of the main repository and run all tests again.
   - Do __NOT__ proceed further until all tests pass.
9. Push to the origin.
10. Create a new `Pull Request` to the `master` branch of the main repository with the required details and a sensible `PR` title.
11. If review changes are requested, repeat steps #5-9 until no more changes are requested.

### Development Environment Setup

Make sure the pre-requisites are satified before proceeding further.

__DO NOT__ use the `nightly` build of Rust for this. We cannot vouch for any behaviour due to differences between the `stable` and `nightly` builds.

#### Pre-Requisites

1. __Rust Compiler__
   - _v1.6+_ __(stable)__
2. __BASH__
   - _GitBash for Windows_
3. __Cargo__

#### Steps

1. `chmod +x scripts/*` to give all scripts in the `scripts` directory permission to execute.

    - You really should go through the scripts before doing this; just good practice.
2. `sh scripts/build.sh both` to build both, the `release` and `debug` versions of the library.
3. `cargo test --verbose` to make sure everything was copied correctly and is working as intended.

## Credits

(ɔ) 2023 [Arkiralor](https://www.github.com/Arkiralor) ([Prithoo Medhi](mailto:prithoo11335@gmail.com))

[^1]: _Takes approximately 7 minutes to check 6'160 values beyond the last element of `prime_checker::libs::constants::KNOWN_ANTIPRIMES`._
