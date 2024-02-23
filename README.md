A Rust implementation of the Whitespace-separated values, as defined by Stenway. Follow the release notes on https://honestlysam.uk/

# 0.1.4
### Friday 23rd February 2024
Adds an implementation of the parser using pest. It took all morning to figure out how to handle an error with a Generic Type parameter, but eventually realised you can just use Display on the whole thing and treat it as a string.

All tests pass once again, and also removed tests concerning parsing the file into valid UTF-8. This will be handled in another crate when I implement the ReliableTXT spec.