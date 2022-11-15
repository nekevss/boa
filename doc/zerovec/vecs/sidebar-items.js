window.SIDEBAR_ITEMS = {"enum":[["FlexZeroVec","A zero-copy data structure that efficiently stores integer values."]],"struct":[["FlexZeroSlice","A zero-copy “slice” that efficiently represents `[usize]`."],["FlexZeroVecOwned","The fully-owned variant of [`FlexZeroVec`]. Contains all mutation methods."],["Index16","This is a [`VarZeroVecFormat`] that stores u16s in the index array. Will have a smaller data size, but it’s more likely for larger arrays to be unrepresentable (and error on construction)"],["Index32","This is a [`VarZeroVecFormat`] that stores u32s in the index array. Will have a larger data size, but will support large arrays without problems."],["VarZeroVecOwned","A fully-owned [`VarZeroVec`]. This type has no lifetime but has the same internal buffer representation of [`VarZeroVec`], making it cheaply convertible to [`VarZeroVec`] and [`VarZeroSlice`]."]],"trait":[["VarZeroVecFormat","This trait allows switching between different possible internal representations of VarZeroVec."]]};