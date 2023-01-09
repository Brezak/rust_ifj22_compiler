# Types
The IFJ22 language currently only supports the types string, int, float, null.
Null is not a type that can be used on its own instead a different type may be marked as nullable meaning that a value
of such type may contain any value of such type or a special value NULL.

## Strings
Strings are stored as series of Unicode scalar values. All strings must be valid utf-8. The IFJ22 language doesn't support
array indexing instead the inbuilt function `substring` should be used

## Integers
Integers are stored in 64 bits using signed two's complement as such their minimum value is -2<sup>63</sup> and maximum values
is 2<sup>63</sup> - 1

## Floats
Floats are stored using the IEEE 754-2008 binary64 format.

> Type:\
> `string` | `float` | `int`
> 
> NullableType:\
> ?_Type_