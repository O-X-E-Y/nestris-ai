use std::ops::Index;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ArrayVec<T, const CAP: usize> {
    inner: [T; CAP],
    len: usize,
}

impl<T: std::fmt::Debug, const CAP: usize> std::fmt::Debug for ArrayVec<T, CAP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.inner[..self.len]).finish()
    }
}

impl<T: Copy, const CAP: usize> ArrayVec<T, CAP> {
    pub const fn new(default: T) -> Self {
        Self {
            inner: [default; CAP],
            len: 0,
        }
    }

    pub const fn new_const(default: T) -> Self {
        Self::new(default)
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn push(&mut self, item: T) {
        if self.len < CAP {
            self.inner[self.len] = item;
            self.len += 1;
        } else {
            panic!("Array capacity exceeded :(");
        }
    }

    pub const fn remove_last(&mut self) {
        self.len = self.len.saturating_sub(1);
    }

    pub const fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                core::hint::assert_unchecked(self.len < CAP);
                Some(std::ptr::read(self.inner.as_ptr().add(self.len)))
            }
        }
    }

    pub const fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len() {
            None
        } else {
            Some(&self.inner[idx])
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntoIter<T, const CAP: usize> {
    inner: [T; CAP],
    len: usize,
    index: usize,
}

impl<T, const CAP: usize> Iterator for IntoIter<T, CAP> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        unsafe {
            core::hint::assert_unchecked(self.index < self.len);
            let res = std::ptr::read(self.inner.as_ptr().add(self.index));
            self.index += 1;
            Some(res)
        }
    }
}

impl<T, const CAP: usize> IntoIterator for ArrayVec<T, CAP> {
    type Item = T;

    type IntoIter = IntoIter<T, CAP>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner,
            len: self.len,
            index: 0,
        }
    }
}

impl<T, const CAP: usize> Index<usize> for ArrayVec<T, CAP> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}
