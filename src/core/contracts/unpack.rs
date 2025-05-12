pub trait Unpack {
    type Output<'a>
    where
        Self: 'a;
    fn unpack(&self) -> Self::Output<'_>;
}
