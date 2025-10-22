use std::collections::VecDeque;

pub struct SingleBufferIter<T: Iterator> {
    inner: T,
    buffer: Option<T::Item>,
}

impl<T: Iterator> SingleBufferIter<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: None,
        }
    }

    pub fn try_put_back(&mut self, value: T::Item) -> Result<(), T::Item> {
        match self.buffer.is_some() {
            true => Err(value),
            false => {
                self.buffer = Some(value);
                Ok(())
            }
        }
    }
}

impl<T: Iterator> Iterator for SingleBufferIter<T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.take().or(self.inner.next())
    }
}

pub struct MultiBufferIter<T: Iterator> {
    inner: T,
    buffer: VecDeque<T::Item>,
}

impl<T: Iterator> MultiBufferIter<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: VecDeque::new(),
        }
    }

    pub fn put_back(&mut self, value: T::Item) {
        self.buffer.push_back(value);
    }
}

impl<T: Iterator> Iterator for MultiBufferIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop_front().or(self.inner.next())
    }
}
