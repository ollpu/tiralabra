//! Miscellaneous array utilities.

/// Shift the contents of the given `array` left, i.e. towards negative indices, and replace the
/// empty positions with the contents of `replace`. The amount to be shifted left is determined by
/// `replace.len()`.
///
/// For example, having `array = [a, b, c, d], replace = [X, Y]` would result in
/// `array = [c, d, X, Y]`.
pub fn shift_left<T: Copy>(array: &mut [T], replace: &[T]) {
    let retain_size = array.len().saturating_sub(replace.len());
    for index in 0..retain_size {
        array[index] = array[index + replace.len()];
    }
    for (slot, item) in array[retain_size..].iter_mut().zip(replace) {
        *slot = *item;
    }
}

/// Similar to [`shift_left`], but in the opposite direction. The contents of `replace` are placed
/// in the beginning of `array`.
pub fn shift_right<T: Copy>(array: &mut [T], replace: &[T]) {
    for index in (replace.len()..array.len()).rev() {
        array[index] = array[index - replace.len()];
    }
    for (slot, item) in array.iter_mut().zip(replace) {
        *slot = *item;
    }
}

/// Like [`shift_left`], but shifts by a given `amount` and fills outside with the value `set`.
pub fn shift_left_fill<T: Copy>(array: &mut [T], amount: usize, set: T) {
    let retain_size = array.len().saturating_sub(amount);
    for index in 0..retain_size {
        array[index] = array[index + amount];
    }
    for slot in &mut array[retain_size..] {
        *slot = set;
    }
}

/// Like [`shift_right`], but shifts by a given `amount` and fills outside with the value `set`.
pub fn shift_right_fill<T: Copy>(array: &mut [T], amount: usize, set: T) {
    for index in (amount..array.len()).rev() {
        array[index] = array[index - amount];
    }
    for slot in &mut array[..amount] {
        *slot = set;
    }
}

/// Iterate over fixed size windows.
///
/// For example, iterating over a sequence `a, b, c, d, e` with `N = 3` would yield
/// `[a, b, c], [b, c, d], [c, d, e]`.
pub struct IterWindows<T, I, const N: usize> {
    source: I,
    array: [T; N],
}

impl<T: Copy + Default, I: Iterator<Item = T>, const N: usize> IterWindows<T, I, N> {
    /// Construct an `IterWindows` from another iterator.
    pub fn from(mut source: I) -> Self {
        let mut array = [Default::default(); N];
        for (slot, item) in array[1..].iter_mut().zip(&mut source) {
            *slot = item;
        }
        Self { source, array }
    }
}

impl<T: Copy + Default, I: Iterator<Item = T>, const N: usize> Iterator for IterWindows<T, I, N> {
    type Item = [T; N];
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.source.next() {
            shift_left(&mut self.array, &[item]);
            Some(self.array)
        } else {
            None
        }
    }
}
