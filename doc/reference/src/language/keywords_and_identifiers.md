# Keywords and Identifiers

## Keywords

The IFJ22 language reserves several keywords that can't be used as identifiers. Most keywords are currently in use but 
several are reserved and may at any point be introduced into the language. What follows is a list of every keyword
split into two groups `Keywords` and `Reserved keywords`
> Keywords:
> - if
> - else
> - elseif
> - while
> - for
> - break
> - continue
> - function
> - float
> - int
> - null
> - string
> - true
> - false
> 
> Reserved keywords:
> - struct
> - class
> - abstract
> - include
> - global
> - private
> - local

## Identifiers
Identifiers are names used currently to identify functions and variables. They cannot be keywords and are composed 
of a series of Ascii alphanumeric characters and the `_` symbol. An identifier must start with a letter or the symbol `_`
but must not be composed entirely of the `_` symbol or numeric characters. A variable identifier is an identifier
prefixed with the `$` symbol.
