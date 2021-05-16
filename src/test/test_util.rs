use crate::util::{shift_left, shift_left_fill, shift_right, shift_right_fill, IterWindows};

#[test]
fn shift_left_example() {
    let mut array = ['a', 'b', 'c', 'd'];
    let replace = ['X', 'Y'];
    shift_left(&mut array, &replace);
    assert_eq!(array, ['c', 'd', 'X', 'Y']);
}

#[test]
fn shift_left_one() {
    let mut array = [1, 2, 3, 4, 5];
    let replace = [-1];
    shift_left(&mut array, &replace);
    assert_eq!(array, [2, 3, 4, 5, -1]);
}

#[test]
fn shift_right_example() {
    let mut array = ['a', 'b', 'c', 'd'];
    let replace = ['X', 'Y'];
    shift_right(&mut array, &replace);
    assert_eq!(array, ['X', 'Y', 'a', 'b']);
}

#[test]
fn shift_left_fill_example() {
    let mut array = ['a', 'b', 'c', 'd'];
    shift_left_fill(&mut array, 2, '-');
    assert_eq!(array, ['c', 'd', '-', '-']);
}

#[test]
fn shift_right_fill_example() {
    let mut array = ['a', 'b', 'c', 'd'];
    shift_right_fill(&mut array, 2, '-');
    assert_eq!(array, ['-', '-', 'a', 'b']);
}

#[test]
fn iter_windows_example() {
    let array = "abcde";
    let iterator = IterWindows::from(array.chars());
    let result: Vec<_> = iterator.collect();
    assert_eq!(result, [['a', 'b', 'c'], ['b', 'c', 'd'], ['c', 'd', 'e']]);
}
