/*
 * Copyright 2023 taylor.fish <contact@taylor.fish>
 *
 * This file is part of integral_constant.
 *
 * integral_constant is licensed under the Apache License, Version 2.0
 * (the "License"); you may not use integral_constant except in compliance
 * with the License. You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! This crate provides type-level representations of constant values. The name
//! `integral_constant` is a reference to [`std::integral_constant`][0] in C++,
//! which serves a similar purpose. Unlike [`std::integral_constant`][0], this
//! crate provides separate wrapper types depending on the type of the constant
//! value, since the type of const generics in Rust cannot depend on type
//! parameters.
//!
//! [0]: https://en.cppreference.com/w/cpp/types/integral_constant

#![no_std]

macro_rules! generate {
    ($article:literal $upper:ident $param:ident $lower:ident) => {
        #[doc = $article]
        #[doc = concat!("[`", stringify!($lower), "`]")]
        /// represented as a type.
        pub struct $upper<const $param: $lower>;

        impl<const $param: $lower> $upper<$param> {
            /// The value of this
            #[doc = concat!("[`", stringify!($lower), "`].")]
            pub const VALUE: $lower = $param;
        }

        impl<const $param: $lower> sealed::Sealed for $upper<$param> {}

        impl<const $param: $lower> Constant<$lower> for $upper<$param> {
            const VALUE: $lower = $param;
        }
    };
}

generate!("A" U8 N u8);
generate!("A" U16 N u16);
generate!("A" U32 N u32);
generate!("A" U64 N u64);
generate!("A" U128 N u128);
generate!("An" I8 N i8);
generate!("An" I16 N i16);
generate!("An" I32 N i32);
generate!("An" I64 N i64);
generate!("An" I128 N i128);
generate!("A" Usize N usize);
generate!("An" Isize N isize);
generate!("A" Bool B bool);
generate!("A" Char C char);

mod sealed {
    pub trait Sealed {}
}

/// A type-level constant.
pub trait Constant<T>: sealed::Sealed {
    /// The value of the constant.
    const VALUE: T;
}
