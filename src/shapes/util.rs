use num_traits::Num;
use num_traits::Zero;

pub trait DiffIteratorExt<I>
where
    I: Iterator,
    I::Item: Num + Zero + Copy
{
    fn diff(self) -> DiffIter<I>;
}

impl<I> DiffIteratorExt<I> for I
where
    I: Iterator,
    I::Item: Num + Zero + Copy
{
    fn diff(self) -> DiffIter<Self> {
        DiffIter::new(self)
    }
}

pub struct DiffIter<I>
where
    I: Iterator,
    I::Item: Num + Zero + Copy,
{
    iter: I,
    prev: I::Item,
}

impl<I> DiffIter<I>
where
    I: Iterator,
    I::Item: Num + Zero + Copy
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            prev: I::Item::zero()
        }
    }
}

impl<I> Iterator for DiffIter<I>
where
    I: Iterator,
    I::Item: Num + Zero + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(next) => {
                let prev = self.prev;
                self.prev = next;

                Some(next - prev)
            }
            None => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::DiffIteratorExt;

    #[test]
    fn simple_diff() {
        let vals: Vec<_> = (1..10).diff().collect();
        assert_eq!(vals, vec![1; 9])
    }
}
