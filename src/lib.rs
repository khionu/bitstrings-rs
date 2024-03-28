#![feature(slice_pattern)]

pub struct Bitstring<'a> {
    raw: &'a [u8],
    #[allow(dead_code)]
    len: usize,
}

pub struct BitstringPattern<'a> {
    data: Bitstring<'a>,
    #[allow(dead_code)]
    pat: (),
}

impl<'a> core::slice::SlicePattern for BitstringPattern<'a> {
    type Item = u8;

    fn as_slice(&self) -> &[Self::Item] {
        self.data.raw
    }
}
