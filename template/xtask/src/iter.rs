pub(crate) trait TryIterator<T, E>: Iterator<Item = Result<T, E>> {
    fn map_ok<U, F>(self, f: F) -> MapOk<Self, F>
    where
        Self: Sized,
        F: FnMut(T) -> U,
    {
        MapOk { iter: self, f }
    }

    fn map_err<G, F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(E) -> G,
    {
        MapErr { iter: self, f }
    }

    fn filter_map_ok<U, F>(self, f: F) -> FilterMapOk<Self, F>
    where
        Self: Sized,
        F: FnMut(T) -> Option<U>,
    {
        FilterMapOk { iter: self, f }
    }

    fn and_then<U, F>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: FnMut(T) -> Result<U, E>,
    {
        AndThen { iter: self, f }
    }
}

impl<I, T, E> TryIterator<T, E> for I where I: Iterator<Item = Result<T, E>> {}

#[derive(Debug)]
pub(crate) struct MapOk<I, F> {
    iter: I,
    f: F,
}

impl<I, T, U, E, F> Iterator for MapOk<I, F>
where
    I: TryIterator<T, E> + Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> U,
{
    type Item = Result<U, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.map(&mut self.f))
    }
}

#[derive(Debug)]
pub(crate) struct MapErr<I, F> {
    iter: I,
    f: F,
}

impl<I, T, E, G, F> Iterator for MapErr<I, F>
where
    I: TryIterator<T, E> + Iterator<Item = Result<T, E>>,
    F: FnMut(E) -> G,
{
    type Item = Result<T, G>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.map_err(&mut self.f))
    }
}

#[derive(Debug)]
pub(crate) struct FilterMapOk<I, F> {
    iter: I,
    f: F,
}

impl<I, T, U, E, F> Iterator for FilterMapOk<I, F>
where
    I: TryIterator<T, E> + Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> Option<U>,
{
    type Item = Result<U, E>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next()? {
                Ok(item) => match (self.f)(item) {
                    Some(item) => return Some(Ok(item)),
                    None => continue,
                },
                Err(err) => return Some(Err(err)),
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct AndThen<I, F> {
    iter: I,
    f: F,
}

impl<I, T, U, E, F> Iterator for AndThen<I, F>
where
    I: TryIterator<T, E> + Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> Result<U, E>,
{
    type Item = Result<U, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.and_then(&mut self.f))
    }
}
