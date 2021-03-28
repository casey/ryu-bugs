use std::iter::Peekable;

#[cfg(test)]
pub fn with_next<Iter>(iter: Iter) -> WithNext<Iter>
where
    Iter: Iterator,
{
    WithNext(iter.peekable())
}

pub struct WithNext<Iter: Iterator>(Peekable<Iter>);

impl<T, Iter> Iterator for WithNext<Iter>
where
    Iter: Iterator<Item = T>,
    T: Clone,
{
    type Item = (<Iter as Iterator>::Item, Option<<Iter as Iterator>::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next();
        match next {
            Some(next) => {
                let following = self.0.peek();
                Some((next, following.cloned()))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterates_over_elements_of_an_iterator_and_the_following_element() {
        let vector = vec![1, 2, 3];
        let result: Vec<_> = with_next(vector.into_iter()).collect();
        assert_eq!(result, vec![(1, Some(2)), (2, Some(3)), (3, None)]);
    }
}
