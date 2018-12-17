use std::marker::PhantomData;
use std::ops::Deref;

pub trait RawIterator {
    type View;

    fn is_end(&self) -> bool;
    fn next(&mut self);

    fn as_view(&self) -> Self::View;

    fn release(&mut self);
}

pub struct CIterator<R>
where
    R: RawIterator,
{
    pub first: bool,
    pub raw: R,
}

impl<R> Drop for CIterator<R>
where
    R: RawIterator,
{
    fn drop(&mut self) {
        self.raw.release();
    }
}

pub struct Borrowed<'i, R>
where
    R: 'i + RawIterator,
{
    it: PhantomData<&'i CIterator<R>>,
    val: R::View,
}

impl<'i, R> Deref for Borrowed<'i, R>
where
    R: RawIterator,
{
    type Target = R::View;

    fn deref(&self) -> &R::View {
        &self.val
    }
}

impl<R> CIterator<R>
where
    R: RawIterator,
{
    pub fn next(&mut self) -> Option<Borrowed<R>> {
        if self.raw.is_end() {
            return None;
        }

        if !self.first {
            self.raw.next();
        }

        self.first = false;

        // we don't want to observe the end marker
        if self.raw.is_end() {
            None
        } else {
            Some(Borrowed {
                it: PhantomData,
                val: self.raw.as_view(),
            })
        }
    }

    pub fn map<F, B>(self, f: F) -> CMap<R, F>
    where
        F: FnMut(&R::View) -> B,
    {
        CMap { it: self, f }
    }

    pub fn filter_map<F, B>(self, f: F) -> CFilterMap<R, F>
    where
        F: FnMut(&R::View) -> Option<B>,
    {
        CFilterMap { it: self, f }
    }

    pub fn any<F>(mut self, mut f: F) -> bool
    where
        F: FnMut(&R::View) -> bool,
    {
        while let Some(view) = self.next() {
            if (f)(&view) {
                return true;
            }
        }

        false
    }

    pub fn all<F>(mut self, mut f: F) -> bool
    where
        F: FnMut(&R::View) -> bool,
    {
        while let Some(view) = self.next() {
            if !(f)(&view) {
                return false;
            }
        }

        true
    }

    pub fn count(mut self) -> usize {
        // Not sure this is actually better than self.map(|_| ()).count()

        let mut count = 0;

        while !self.raw.is_end() {
            self.raw.next();
            count += 1;
        }

        count
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct CMap<R, F>
where
    R: RawIterator,
{
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

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct CFilterMap<R, F>
where
    R: RawIterator,
{
    it: CIterator<R>,
    f: F,
}

impl<B, R, F> Iterator for CFilterMap<R, F>
where
    R: RawIterator,
    F: FnMut(&R::View) -> Option<B>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.it.next() {
                Some(ref x) => {
                    if let Some(y) = (self.f)(x) {
                        return Some(y);
                    }
                }
                None => return None,
            }
        }
    }
}
