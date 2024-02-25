A Rust implementation of the Whitespace-separated values, as defined by Stenway. Follow the release notes on https://honestlysam.uk/

# 0.1.5
### Sunday 25th February 2024
Rewrote the pest grammar from scratch which simplified the code processing the parser output. Made public the tabulate module so the binary can be executed to demonstrate the function and allow for a visual representation in CLI of the parser functioning properly. Useful for debugging. Introduced [thiserror](https://crates.io/crate/thiserror/) for the pest implementation.

TODO:
1. Process command line arguments to take a file path, rather than hardcoding one
2. Switch integration tests to unit tests or doc tests so I can remove the public access and elect one implementation to import through wsv::parse.
3. Create a new parser using [nom](https://crates.io/crate/nom/) 
4. Write documentation.

# 0.1.4
### Friday 23rd February 2024
Adds an implementation of the parser using pest. It took all morning to figure out how to handle an error with a Generic Type parameter, but eventually realised you can just use Display on the whole thing and treat it as a string.

All tests pass once again, and also removed tests concerning parsing the file into valid UTF-8. This will be handled in another crate when I implement the ReliableTXT spec.