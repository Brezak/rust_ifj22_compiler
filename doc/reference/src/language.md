# The language
IFJ22 is a subset of the PHP language and as such any valid IFJ22 code can be run using the PHP interpreter.
The major differences are in no particular order:
- No support for types other than booleans, integers, floats, null and strings
- No support for arrays and array indexing
- Basic support only for while loops
- No support for function calls in expressions

This compiler supports a modified version of the IFJ22 language which adds support for function calls in expressions,
changes the underlying integer type from a 32-bit signed integer into a 64-bit signed integer[^integer sizes],
adds for loops and other niceties[^additional features].

## Grammar

The following subchapters will be the reference for the language this compiler works with.
The notation and structure used in this reference will be based on the [Rust Reference](https://doc.rust-lang.org/stable/reference/notation.html)

[^integer sizes]: The underlying floating point type for floats is 64 bits. I found no reason why integers shouldn't be just as wide.

[^additional features]: The original IFJ22 language specification mentions extensions to the language e.g. for loops
this compiler implements all of them
