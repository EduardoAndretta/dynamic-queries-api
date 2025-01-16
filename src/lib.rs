pub struct EventfulPeekable<I>
where
    I: Iterator,
{
    inner: std::iter::Peekable<I>,
    on_next: Box<dyn FnMut(&I::Item)>,
}

impl<I> EventfulPeekable<I>
where
    I: Iterator,
{
    pub fn new(iter: I, on_next: impl FnMut(&I::Item) + 'static) -> Self {
        EventfulPeekable {
            inner: iter.peekable(),
            on_next: Box::new(on_next),
        }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.inner.peek()
    }
}

impl<I> Iterator for EventfulPeekable<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.inner.next();
        if let Some(ref item) = next_item {
            (self.on_next)(item);
        }
        next_item
    }
}
