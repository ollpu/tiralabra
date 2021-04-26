use crate::ring_buffer;

#[test]
fn pop_reads_correct_data() {
    let (mut tx, mut rx) = ring_buffer::with_capacity(4);
    let correct = [4i32, -1, 4, 149];
    tx.push(&correct).unwrap();
    let mut result = vec![0; 4];
    rx.pop_full(&mut result).unwrap();
    assert_eq!(result[..], correct[..]);
}

#[test]
fn cannot_push_more_than_capacity() {
    let (mut tx, mut rx) = ring_buffer::with_capacity(4);
    tx.push(&[1i32, 5, 23]).unwrap();
    let result = tx.push(&[15, 44]);
    assert_eq!(result, Err(1));
    rx.pop_full(&mut [0; 4]).unwrap();
    let result = tx.push(&[1, 2, 3, 4, 5]);
    assert_eq!(result, Err(4));
    let result = tx.push(&[1, 2, 3, 4, 5]);
    assert_eq!(result, Err(0));
}

#[test]
fn cannot_pop_more_than_available() {
    let (mut tx, mut rx) = ring_buffer::with_capacity(4);
    tx.push(&[1i32, 2, 3]).unwrap();
    let mut array = [0; 4];
    let result = rx.pop_full(&mut array);
    assert_eq!(result, Err(3));
    assert_eq!(array, [0; 4]);
}

#[test]
fn all_possible_read_write_index_combinations() {
    const N: usize = 8;
    let (mut tx, mut rx) = ring_buffer::with_capacity(N);
    for _ in 0..=N {
        for v in 0..N {
            tx.push(&[v]).unwrap();
        }
        let result = tx.push(&[N]);
        assert_eq!(result, Err(0));
        let mut array = [N+1];
        for v in 0..N {
            rx.pop_full(&mut array).unwrap();
            assert_eq!(array[0], v);
        }
        let result = rx.pop_full(&mut array);
        assert_eq!(result, Err(0));
    }
}

#[test]
fn multithreading() {
    let (mut tx, mut rx) = ring_buffer::with_capacity(4);
    let rx_handle = std::thread::spawn(move || {
        for _try in 0..100 {
            let mut array = [0; 4];
            match rx.pop_full(&mut array) {
                Ok(()) => {
                    assert_eq!(array, [1, 2, 3, 4]);
                    return;
                }
                Err(_) => ()
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        panic!();
    });
    let tx_handle = std::thread::spawn(move || {
        tx.push(&[1i64, 2, 3, 4]).unwrap();
    });
    tx_handle.join().unwrap();
    rx_handle.join().unwrap();
}
