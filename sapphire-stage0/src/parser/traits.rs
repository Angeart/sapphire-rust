use nom::{
    Compare, ExtendInto, InputIter, InputLength, InputTake, InputTakeAtPosition, Offset, Slice,
};
use nom_greedyerror::AsStr;
use std::ops::{RangeFrom, RangeTo};

pub trait ParserInput<'a>:
    Clone
    + Compare<&'a str>
    + Slice<RangeFrom<usize>>
    + Slice<RangeTo<usize>>
    + InputIter
    + InputLength
    + InputTake
    + InputTakeAtPosition
    + AsStr
    + Offset
    + PartialEq
    + ExtendInto
    + ExtendInto<Item = char, Extender = String>
// where
//     <Self as InputIter>::Item: AsChar + Copy,
//     <Self as InputTakeAtPosition>::Item: AsChar + Copy,
{
}

impl<
        'a,
        T: Clone
            + Compare<&'a str>
            + Slice<RangeFrom<usize>>
            + Slice<RangeTo<usize>>
            + InputIter
            + InputLength
            + InputTake
            + InputTakeAtPosition
            + AsStr
            + Offset
            + PartialEq
            + ExtendInto
            + ExtendInto<Item = char, Extender = String>,
    > ParserInput<'a> for T
// where
//     <T as InputIter>::Item: AsChar + Copy,
//     <T as InputTakeAtPosition>::Item: AsChar + Copy,
{
}
