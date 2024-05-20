# 0.5.0

### Monday 20th May 2024

Fleshed out the README.md with much more detail about my objective with this crate and progress. To summarise:

> This repository is an exercise in all the features of the Rust programming language, as applied to a use case which is complex enough to actually need most of them, but not so complex that it takes hours to understand and implement each one. It is also my attempt to demonstrate in code the various design principles I have collected on my journey to becoming an artisan engineer. It is my masterpiece, without the Masters. I plan to demonstrate my best efforts in static, streaming, parallel and async computation, benchmarking, testing and documentation.
> 
> It is also to demonstrate the use of every parsing crate I can get my hands on, applied to just one format, so that a casual reader may use it to help decide which one to use for themselves. The applications of this format are beside the point. The value in choosing this format is that there is currently no other crate to handle this, and that it is simple enough that I can implement lots of variants.

Switched benchmarking frameworks from Criterion to Divan. Starting with Criterion was a good decision, since it informed what I should expect from Divan. Thank you Criterion devs. Divan facilitates keeping the benchmark code closer to the source code, so it took some experimenting to settle on the current structure. Also added `Mealy` and `Moore` state machine implementations, which are slower than my first state machine, but serve as a solid demonstration of each concept. Still need to polish them to be eligible for teaching.

`Nom` remains the second fastest, behind `State`, but still doesn't have all error features, and so I discount the result from the benchmark until it is fully featured.

Keeping all notes in this repo now. Created an `IO` module to contain uses of the parse functions which are then the subject of benchmarking. Added a `parse_strict` variant which takes on the functionality of the old `parse`, and now `parse` returns a `Vec` of `Result`s, where `parse_strict` returns a `Result` of `Vec`s. Possibly better to name them `parse` and `parse_lossy`. Not sure.

TODO:

1. `Serde` implementation (easier now, with focus on the line over the file)
2. Handle `nom` errors better.
3. Improve Mealy and Moore docs and impl for teaching.

# 0.4.0

### Friday 10th May 2024

Added a new parser implementation, using a state machine. The current impl is how I naturally added it, but I am going to refactor it into a form more closely representing a state machine for educational purposes. Rebuilt the Error type for all parsers. Still working out how to integrate the Pest and Nom errors more smoothly. Will investigate nom-supreme for this.

TODO:

1. Serde implementation (easier now, with focus on the line over the file)
2. ~~Shift integration tests to unit or doc tests.~~
3. ~~Benchmarking: Learn Divan~~
4. Handle nom errors better.
5. Augment the state machine impl for teaching.
5. ~~Other documentation.~~


# 0.3.2

### Sunday 28th April 2024

Fixed trailing whitespace bug with primitive. Renamed primitive2 to split. Added more tests. Feature gated the nom and pest parsers behind the feature gates "nom" and "pest".

TODO:

1. Serde implementation
2. ~~Shift focus to line parsing, in prep for stream parsing.~~
2. Shift integration tests to unit or doc tests. Integration tests need to be different now, since serde does not have the halfway point I am currently testing to.
3. Benchmarking: Learn Divan
4. Learn how to handle errors better.
5. Documentation


### Friday 26th April 2024

Fixed bugs with primitive2 and pest. Added more tests, and added benchmarks with Criterion. The results demonstrate that primitive and nom are about as fast as each other, with nom a touch faster, primitive2 is over 50% slower than primitive, and pest is anywhere between 80% and 300% slower than primitive.

TODO:

1. ~~Fix error tests on nom and pest. Learn how to handle errors better.~~
2. ~~feature gates for all parsers.~~
3. Serde implementation
4. Shift integration tests to unit or doc tests
5. Benchmarking: Learn Divan
6. Documentation

# 0.3.1

### Wednesday 13th March 2024

Added implementation of primitive2 after a lot of pain and heartache. Added dependency to itertools for it. The form is very similar to the primitive implementation but the detail is far worse. I wonder if there is a mathematical equivalence somewhere. Read up on Serde implementations of other data formats and prepped for that stage. Tidied up integration tests.

Produced a somewhat large file to do benchmarking against. Now need to learn one of those frameworks. Divan looks good. I will compress my sample inputs into const strings in one file, so I remove all I/O from the integration tests and benchmarks.

Added minimal CLI support to take integration test files only. Panics otherwise.

TODO:

1. Fix error tests on nom and pest. Learn how to handle errors better.
2. feature gates for all parsers.
3. Serde implementation
4. Benchmarking: Learn Divan ~~or Criterion~~
5. Documentation
6. Shift integration tests to unit or doc tests

# 0.3.0

### Monday 11th March 2024

Modified the type of all parsers to return a Result<Wsv, WsvError> so that I could implement custom traits on the Wsv newtype. The tabulate functionality is now Display, and there are also Default, IntoIterator, PartialEq and From. This will allow me to intentionally control the functionality of the resulting type.

Shifted around the integration test code so that it didn't use macros and was easier to feature gate (coming soon). Shifted around the source code to practice using modules and privacy settings, and found that lib cannot declare a directory for sub-modules.

Added another primitive implementation of the parser which hinges on splitting the string and dealing with the parts, rather than iterating over every character in order. This is unfinished.

I also played with the tracing crate, which was somewhat useful for debugging but I still haven't baked it into the implementations. Not sure where would be best.

TODO:

1. ~~CLI interface~~
2. Serde implementation
3. ~~Benchmarking: Produce a huge file for this.~~
4. Documentation
5. Shift integration tests to unit or doc tests

# 0.2.0

### Tuesday 27th February 2024

Added an implementation of the parser using nom and exposed wsv::parse as a common interface. Unified all the test cases
and standardised the API of all three implementations, up to errors. Also unified the data model underlying all three.

I will publish 1.0.0 once I have successfully feature-gated the alternative functions.

TODO:

1. Shift all the code I have for the ReliableTXT implementation somewhere else
2. Process command line arguments to take a file path with [clap](https://crates.io/crate/clap/)
   or [clio](https://crates.io/crate/clio/).

# 0.1.5

### Sunday 25th February 2024

Rewrote the pest grammar from scratch which simplified the code processing the parser output. Made public the tabulate
module so the binary can be executed to demonstrate the function and allow for a visual representation in CLI of the
parser functioning properly. Useful for debugging. Introduced [thiserror](https://crates.io/crate/thiserror/) for the
pest implementation.

TODO:

1. Process command line arguments to take a file path, rather than hardcoding one
2. Switch integration tests to unit tests or doc tests, so I can remove the public access and elect one implementation
   to import through wsv::parse.
3. Create a new parser using [nom](https://crates.io/crate/nom/)
4. Write documentation.

# 0.1.4

### Friday 23rd February 2024

Adds an implementation of the parser using pest. It took all morning to figure out how to handle an error with a Generic
Type parameter, but eventually realised you can just use Display on the whole thing and treat it as a string.

All tests pass once again, and also removed tests concerning parsing the file into valid UTF-8. This will be handled in
another crate when I implement the ReliableTXT spec.
