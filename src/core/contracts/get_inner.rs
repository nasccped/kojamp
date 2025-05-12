pub trait GetInner {
    type Output<'a>
    where
        Self: 'a;
    fn get_inner(&self) -> Self::Output<'_>;
}
