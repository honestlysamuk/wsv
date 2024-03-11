A Rust implementation of the Whitespace-separated values, as defined by Stenway. Follow the release notes
on https://honestlysam.uk/

# 0.3.0

### Monday 11th March 2024

Modified the type of all parsers to return a Result<Wsv, WsvError> so that I could implement custom traits on the Wsv newtype. The tabulate functionality is now Display, and there are also Default, IntoIterator, PartialEq and From. This will allow me to intentionally control the functionality of the resulting type.

Shifted around the integration test code so that it didn't use macros and was easier to feature gate (coming soon). Shifted around the source code to practice using modules and privacy settings, and found that lib cannot declare a directory for sub-modules.

Added another primitive implementation of the parser which hinges on splitting the string and dealing with the parts, rather than iterating over every character in order. This is unfinished.

I also played with the tracing crate, which was somewhat useful for debugging but I still haven't baked it into the implementations. Not sure where would be best.

TODO:

1. CLI interface
2. Serde implementation
3. Benchmarking: Produce a huge file for this.
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