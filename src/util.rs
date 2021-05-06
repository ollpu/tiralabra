
/// Shift the contents of the given `array` left, i.e. towards negative indices, and replace the
/// empty positions with the contents of `replace`. The amount to be shifted left is determined by
/// `replace.len()`.
///
/// For example, having `array = [a, b, c, d], replace = [X, Y]` would result in
/// `array = [c, d, X, Y]`.
pub fn shift_left<T: Clone>(array: &mut [T], replace: &[T]) {
    let retain_size = array.len().saturating_sub(replace.len());
    for index in 0..retain_size {
        array[index] = array[index + replace.len()].clone();
    }
    for (slot, item) in array[retain_size..].iter_mut().zip(replace) {
        *slot = item.clone();
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

impl<T: Copy + Default, I: Iterator<Item=T>, const N: usize> IterWindows<T, I, N>
{
    /// Construct an `IterWindows` from another iterator.
    pub fn from(mut source: I) -> Self {
        let mut array = [Default::default(); N];
        for (slot, item) in array[1..].iter_mut().zip(&mut source) {
            *slot = item;
        }
        Self { source, array }
    }
}

impl<T: Copy + Default, I: Iterator<Item=T>, const N: usize> Iterator for IterWindows<T, I, N> {
    type Item = [T; N];
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.source.next() {
            shift_left(&mut self.array, &[item]);
            Some(self.array.clone())
        } else {
            None
        }
    }
}
