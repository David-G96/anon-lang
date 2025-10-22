//! This mod provides two "put-back-able" iterators: SingleBufferIter and MultiBufferIter

/// A buffer iterator with single buffer, allows you to put back one item.
pub struct SingleBufferIter<T: Iterator> {
    inner: T,
    buffer: Option<T::Item>,
}

impl<T: Iterator> SingleBufferIter<T> {
    /// creates a new SingleBufferIter with the specific Iterator
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: None,
        }
    }

    /// try to put back one item.
    /// returns Ok if the buffer is empty, returns Err and the item if the buffer is occupied.
    pub fn try_put_back(&mut self, value: T::Item) -> Result<(), T::Item> {
        match self.buffer.is_some() {
            true => Err(value),
            false => {
                self.buffer = Some(value);
                Ok(())
            }
        }
    }

    pub fn try_put_back_option(
        &mut self,
        opt_value: Option<T::Item>,
    ) -> Result<(), Option<T::Item>> {
        if let Some(value) = opt_value {
            self.buffer = Some(value);
            Ok(())
        } else {
            Err(opt_value)
        }
    }
}

impl<T: Iterator> Iterator for SingleBufferIter<T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.take().or_else(|| self.inner.next())
    }
}

/// A multi buffer Iterator, allows you to put back multiple items.
pub struct MultiBufferIter<T: Iterator> {
    inner: T,
    buffer: Vec<T::Item>,
}

impl<T: Iterator> MultiBufferIter<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: Vec::new(),
        }
    }

    pub fn put_back(&mut self, value: T::Item) {
        self.buffer.push(value);
    }

    pub fn put_back_opt(&mut self, opt_value: Option<T::Item>) {
        if let Some(v) = opt_value {
            self.put_back(v)
        }
    }
}

impl<T: Iterator> Iterator for MultiBufferIter<T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop().or_else(|| self.inner.next())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_single_buffer() {
        let inner = ["1", "2", "3", "4", "5"].iter().map(|&x| String::from(x));
        let mut single_buffer = SingleBufferIter::new(inner);

        let a = single_buffer.next().unwrap();
        assert_eq!(a.as_str(), "1");
        let b = single_buffer.next().unwrap();
        assert_eq!(b.as_str(), "2");

        // 现在把 b 和 a 移入 try_put_back
        assert_eq!(Ok(()), single_buffer.try_put_back(b));
        assert_eq!(Err("1".to_string()), single_buffer.try_put_back(a));

        let expected = ["2", "3", "4", "5"].map(|x| x.to_string());
        assert_eq!(expected, single_buffer.collect::<Vec<String>>().as_slice());
    }

    #[test]
    fn test_multi_buffer() {
        let inner = ["1", "2", "3", "4", "5"].iter().map(|&x| x.to_string());
        let mut multi_buffer = MultiBufferIter::new(inner);

        let a = multi_buffer.next().unwrap();
        assert_eq!(a.as_str(), "1");
        let b = multi_buffer.next().unwrap();
        assert_eq!(b.as_str(), "2");

        multi_buffer.put_back(b);
        multi_buffer.put_back(a);

        let a = multi_buffer.next().unwrap();
        assert_eq!(a.as_str(), "1");
        let b = multi_buffer.next().unwrap();
        assert_eq!(b.as_str(), "2");

        let expected = ["3", "4", "5"].map(|x| x.to_string());
        assert_eq!(expected, multi_buffer.collect::<Vec<String>>().as_slice());
    }
}
