```wsv
####################################################################
                                                                    
 "8.`888b                 ,8"   d888888o.    "8.`888b           ,8" 
  "8.`888b               ,8"  .`8888:' `88.   "8.`888b         ,8"  
   "8.`888b             ,8"   8.`8888.   Y8    "8.`888b       ,8"   
    "8.`888b     .b    ,8"    `8.`8888.         "8.`888b     ,8"    
     "8.`888b    88b  ,8"      `8.`8888.         "8.`888b   ,8"     
      "8.`888b .`888b,8"        `8.`8888.         "8.`888b ,8"      
       "8.`888b8.`8888"          `8.`8888.         "8.`888b8"       
        "8.`888`8.`88"       8b   `8.`8888.         "8.`888"        
         "8.`8' `8,`"   - -  `8b.  ;8.`8888    - -   "8.`8"    - -  
          "8.`   `8"    - -   `Y8888P ,88P'    - -    "8."     - -  
                                                                    
####################################################################
```

<sup>ASCII credits go to the Broadway font on [patorjk.com](patorjk.com). Thanks!</sup>

# Introduction

This library aims to solve the problem of choice paralysis for parser tooling.

WSV is an educational library, showcasing one format interpreted by every parsing crate existing in the Rust ecosystem. It is a broad church, covering methods appropriate for data ingestion and persistance, programming language lexing and natural language processing tools. The WSV format is a goldilocks format, simple enough to understand in a few minutes, complex enough to house some interesting edge cases.

Join me in my journey to showcase the diversity Rust has to offer the budding parse developer.

Release notes can be found [here](https://github.com/honestlysamuk/wsv/blob/main/release_notes.md).
For more on my wider motivation for this library, see : [here](https://honestlysam.uk/superpower/).

I believe that Rust's greatest superpower is in its educational potential. It is designed to be verbose, explicit and intentional, which cannot help but bleed into the teaching of the language too. Consequently, every parsing library I have found has fantastic documentation and examples of lots of different formats which they handle. However, while this is beneficial for improving your understanding of a single crate, it does not help to choose it in the first place.

This library aims to showcase the many tools we have available, applied to the same data format, so it is easier to choose between them. Each solution differs in how comprehensive, readable, teachable and extensible it is. These qualities are documented on each option. The only metrics we consider are a micro-benchmark and non-whitespace character count in the file.

Consider this to be a library in the original meaning. Peruse the code-shelves and read the blurbs at your leisure. Avoid pigeonholing yourself with a tool that doesn't offer everything you need. This is also a binary crate, so if you choose to install the package, you can run it on a WSV file and have it pretty-printed to the terminal. This will use the fastest implementation available.

N.B. The goal is to showcase a classic, "idiomatic" use of each crate or idea. If you have an idea designed to be the fastest, it would be a welcome addition.

N.M.B. I would also like to showcase the ways this API can be used. The IO module contains functions which interact with the file system.

### Understand the format

WSV is a Whitespace-separated-values file. The following is a valid example demonstrating every feature. This will give you a solid enough understanding to dive into the options already, but if you want more, check out the [spec](https://dev.stenway.com/WSV/Specification.html).

```wsv
    Values are separated by any       Unicode-defined         whitespace   
A "value" "surrounded by double quotes" allows for "whitespace in the value."
There are "two escape characters. "" is a double quote, "/" is a newline."
Comments are indicated with the # character.
Lines are separated by \n characters, not terminated by them. \r isn't special,
but since we ignore whitespace at the beginning and end of each row, splitting
on /n/r is still valid.
Numbers aren't special either. 3 345 546.456. 
```

The ASCII title is also a valid WSV file. If anyone knows how to get GitHub to highlight it, I would be very keen to help make it happen!

# The Next Objective

Implement parsers using the following tools:
1. ***State-like***
   1. Mealy machine (/)
   2. Moore machine (/)
2. ***Combinator-like***
   1. nom (/)
   2. Winnow
3. ***serde-like***
   1. Serde
   2. Rkyv
4. ***grammar-like***
   1. Pest (/)
   2. Oak
   3. Peginator
   4.  rust-peg

# Contributing

This library welcomes contributions of new implementations. There is already a benchmarking and testing suite available to you to code against. It also welcomes tweaks to existing implementations, especially from well-versed users of the crates in question, to shape the solution in a way one would more commonly find in the wild. 

### Serialisation

Currently, the focus is only on text file -> Rust data model. deserialisation, since some tools are uni-directional. This repo welcomes serialisation solutions but note that the surrounding testing does not yet exist.

