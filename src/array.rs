// pub fn push_to_front<const N:usize,T>(array:&mut[T;N],new_value:T)->T {
//     let mut removed_value = new_value;
//     for item_in_array in array {
//         std::mem::swap(&mut removed_value, item_in_array);
//     };
//     removed_value
// }
// pub fn push_to_end<const N:usize,T>(array:&mut[T;N],new_value:T)->T {
//     let mut removed_value = new_value;
//     for item_in_array in array.iter_mut().rev() {
//         std::mem::swap(&mut removed_value, item_in_array);
//     };
//     removed_value
// }

use std::mem::MaybeUninit;
// use std::array::IntoIter;

pub fn get_two_mut<T>(slice: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => {
            let (a_slice, b_slice) = slice.split_at_mut(b);
            Some((&mut a_slice[a], &mut b_slice[0]))
        }
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => {
            let (b_slice, a_slice) = slice.split_at_mut(a);
            Some((&mut a_slice[0], &mut b_slice[b]))
        }
    }
}

type NextChunkError<const N: usize, T> = Vec<T>;
// type NextChunkError<const N:usize,T> = std::array::IntoIter<T,N>;

pub fn next_chunk<const N: usize, I: Iterator>(
    iter: &mut I,
) -> Result<[I::Item; N], NextChunkError<N, I::Item>> {
    let mut chunk = [(); N].map(|_| MaybeUninit::uninit());
    for chunk_index in 0..N {
        if let Some(i) = iter.next() {
            chunk[chunk_index].write(i);
        } else {
            // return Err(unsafe {
            //     IntoIter::new_unchecked(chunk, 0..chunk_index)
            // });
            return Err(chunk
                .into_iter()
                .take(chunk_index)
                .map(|maybe_uninit| unsafe { maybe_uninit.assume_init() })
                .collect());
        }
    }
    Ok(chunk.map(|maybe_uninit| unsafe { maybe_uninit.assume_init() }))
}
