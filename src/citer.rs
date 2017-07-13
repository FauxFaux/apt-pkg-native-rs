pub trait RawIterator {
    type View;

    fn is_end(&self) -> bool;
    fn next(&mut self);

    fn as_view(&self) -> Self::View;

    fn release(&mut self);
}

pub struct CIterator<R>
where R: RawIterator {
    pub first: bool,
    pub raw: R,
}

impl<R> Drop for CIterator<R>
where R: RawIterator {
    fn drop(&mut self) {
        self.raw.release();
    }
}

impl<R> CIterator<R>
where R: RawIterator {
    pub fn next(&mut self) -> Option<R::View> {
        if self.raw.is_end() {
            return None;
        }

        if !self.first {
            self.raw.next();
        }

        self.first = false;

        // we don't want to observe the end marker
        if self.raw.is_end() { None } else { Some(self.raw.as_view()) }
    }

    pub fn map<F, B>(self, f: F) -> CMap<R, F>
    where
        F: FnMut(&R::View) -> B,
    {
        CMap { it: self, f }
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct CMap<R, F>
where R: RawIterator {
    it: CIterator<R>,
    f: F,
}

impl<B, R, F> Iterator for CMap<R, F>
where
    R: RawIterator,
    F: FnMut(&R::View) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        match self.it.next() {
            Some(ref x) => Some((self.f)(x)),
            None => None,
        }
    }
}
