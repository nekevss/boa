window.SIDEBAR_ITEMS = {"enum":[["VarZeroVec","A zero-copy vector for variable-width types."],["ZeroVec","A zero-copy vector for fixed-width types."],["ZeroVecError","A generic error type to be used for decoding slices of ULE types"]],"mod":[["maps","This module contains additional utility types and traits for working with [`ZeroMap`] and [`ZeroMap2d`]. See their docs for more details on the general purpose of these types."],["ule","Traits over unaligned little-endian data (ULE, pronounced “yule”)."],["vecs","This module contains additional utility types for working with [`ZeroVec`] and  [`VarZeroVec`]. See their docs for more details on the general purpose of these types."]],"struct":[["VarZeroSlice","A zero-copy “slice”, that works for unsized types, i.e. the zero-copy version of `[T]` where `T` is not `Sized`."],["ZeroMap","A zero-copy map datastructure, built on sorted binary-searchable [`ZeroVec`] and `VarZeroVec`."],["ZeroMap2d","A zero-copy, two-dimensional map datastructure ."],["ZeroSlice","A zero-copy “slice”, i.e. the zero-copy version of `[T]`. This behaves similarly to [`ZeroVec<T>`], however [`ZeroVec<T>`] is allowed to contain owned data and as such is ideal for deserialization since most human readable serialization formats cannot unconditionally deserialize zero-copy."]]};