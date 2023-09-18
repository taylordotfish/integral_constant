integral\_constant
==================

This crate provides type-level representations of constant values. The name
`integral_constant` is a reference to [`std::integral_constant`][0] in C++,
which serves a similar purpose. Unlike [`std::integral_constant`][0], this
crate provides separate wrapper types depending on the type of the constant
value, since the type of const generics in Rust cannot depend on type
parameters.

[0]: https://en.cppreference.com/w/cpp/types/integral_constant
