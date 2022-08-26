window.SIDEBAR_ITEMS = {"struct":[["AliasesV1","This alias data is used for locale canonicalization. Each field defines a mapping from an old identifier to a new identifier, based upon the rules in from http://unicode.org/reports/tr35/#LocaleId_Canonicalization. The data is stored in sorted order, allowing for binary search to identify rules to apply. It is broken down into smaller vectors based upon some characteristic of the data, to help avoid unnecessary searches. For example, the `sgn_region` field contains aliases for sign language and region, so that it is not necessary to search the data unless the input is a sign language."],["AliasesV1Marker","Marker type for [`AliasesV1`]: “locale_canonicalizer/aliases@1”"],["LikelySubtagsV1","This likely subtags data is used for the minimize and maximize operations. Each field defines a mapping from an old identifier to a new identifier, based upon the rules in https://www.unicode.org/reports/tr35/#Likely_Subtags."],["LikelySubtagsV1Marker","Marker type for [`LikelySubtagsV1`]: “locale_canonicalizer/likelysubtags@1”"],["StrStrPair","A pair of strings with a EncodeAsVarULE implementation."],["StrStrPairVarULE","`VarULE` type for StrStrPair"]]};