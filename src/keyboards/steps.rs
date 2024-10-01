use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// A struct that carries data for a keyboard.
///
/// When changing any kind of setting for a keyboard, the internally stored [`Steps`] is mutated.
/// The data is stored as a single [`std::array`], then split with using the `step_count`, and
/// `step_len` when iteration is being performed. This allows storing multi-packet settings in a unified fashion. This allows reading
/// into all of the packets at once, or iterate over each packet.
#[derive(Clone, Debug)]
pub struct Steps<const DATA_LEN: usize> {
    pub data: [u8; DATA_LEN],
    pub step_count: usize,
    pub step_len: usize,
}

impl<const DATA_LEN: usize> Steps<DATA_LEN> {
    #[inline]
    pub fn new(step_count: usize, step_len: usize, starter_bytes: &[u8]) -> Self {
        let mut data: [u8; DATA_LEN] = [0; DATA_LEN];

        let starter_len = starter_bytes.len();

        (0..step_count).for_each(|i| {
            data[step_len * i..step_len * i + starter_len].clone_from_slice(starter_bytes);
        });

        Self {
            data,
            step_count,
            step_len,
        }
    }

    /// Iterator over each step as mutable.
    #[inline(always)]
    pub fn steps_mut(&mut self) -> impl Iterator<Item = &mut [u8]> {
        self.data.as_mut_slice().chunks_exact_mut(self.step_len)
    }

    /// Iterator over each step.
    #[inline(always)]
    pub fn steps(&self) -> impl Iterator<Item = &[u8]> {
        self.data.as_slice().chunks_exact(self.step_len)
    }

    /// Get the N'th step in the steps.
    ///
    /// # Panics:
    /// Will cause a panic if provided index is out of bounds.
    /// This is means, if only 5 steps exist, `5_u8` will cause a panic.
    #[inline(always)]
    pub fn nth_step(&self, index: usize) -> &[u8] {
        &self.data[self.step_len * index..self.step_len * (index + 1)]
    }

    #[inline(always)]
    pub fn nth_step_mut(&mut self, index: usize) -> &mut [u8] {
        &mut self.data[self.step_len * index..self.step_len * (index + 1)]
    }
}

impl<const DATA_LEN: usize> Index<Indexes> for Steps<DATA_LEN> {
    type Output = u8;

    #[inline(always)]
    fn index(&self, index: Indexes) -> &Self::Output {
        &self.data[self.step_len * index.step + index.index]
    }
}

impl<const DATA_LEN: usize> IndexMut<Indexes> for Steps<DATA_LEN> {
    #[inline(always)]
    fn index_mut(&mut self, index: Indexes) -> &mut Self::Output {
        &mut self.data[self.step_len * index.step + index.index]
    }
}

impl<const DATA_LEN: usize> Deref for Steps<DATA_LEN> {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<const DATA_LEN: usize> DerefMut for Steps<DATA_LEN> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// Indexes for the data that is being processed.
#[derive(Clone, Debug)]
pub struct Indexes {
    /// The N'th step.
    pub step: usize,

    /// The N'th value in the N'th step.
    pub index: usize,
}

impl Indexes {
    #[inline(always)]
    pub(crate) const fn new(step: usize, index: usize) -> Self {
        Self { step, index }
    }
}

/// Create an array of [`Indexes`] with the length of `LEN`.
pub(crate) const fn same_step_indexes<const LEN: usize>(
    step_count: usize,
    first: usize,
) -> [Indexes; LEN] {
    // To create the array of indexes in the const function, we allocate the array as `MaybeUninit`
    // and then transmute it to treat the array as it was intialized. Instead of using `unsafe` we
    // could make `Indexes` `Copy`, but thats not fun.
    //
    // SAFETY:
    // It is safe to transmute here as `Indexes` only contains two `usize`'s.
    let arr: MaybeUninit<[Indexes; LEN]> = MaybeUninit::zeroed();

    let mut arr = unsafe { arr.assume_init() };

    let mut i = 0;
    while i < LEN {
        // It would be better to use `MaybeUninit::write` once it is made const.
        arr[i] = Indexes::new(step_count, first + i);
        i += 1;
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::{same_step_indexes, Indexes};

    #[test]
    fn test_same_step_indexes() {
        let indexes: [Indexes; 5] = same_step_indexes(1, 1);
        for (i, index) in indexes.into_iter().enumerate() {
            assert_eq!(i + 1, index.index);
        }

        let indexes: [Indexes; 20] = same_step_indexes(1, 100);
        for (i, index) in indexes.into_iter().enumerate() {
            assert_eq!(i + 100, index.index);
        }

        let indexes: [Indexes; 50] = same_step_indexes(1, 420);
        for (i, index) in indexes.into_iter().enumerate() {
            assert_eq!(i + 420, index.index);
        }
    }
}
