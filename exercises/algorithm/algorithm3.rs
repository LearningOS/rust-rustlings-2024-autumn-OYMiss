/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

use std::mem::swap;

fn partition<T: std::cmp::PartialOrd>(array: &mut [T], start: usize, end: usize) -> usize {
    let mut i = start + 1;
    let mut j = end - 1;
    if end - start == 1 {
        return start;
    }

    loop {
        while i < j && &array[i] <= &array[start] {
            i += 1;
        }
        while i < j && &array[j] > &array[start] {
            j -= 1;
        }
        if i < j {
            array.swap(i, j);
        } else {
            break;
        }
    }

    if &array[i] <= &array[start] {
        array.swap(i, start);
        return i;
    } else {
        array.swap(i - 1, start);
        return i - 1;
    }
}

fn partition2<T: std::cmp::PartialOrd>(array: &mut [T], start: usize, end: usize) -> usize {
    if end - start == 1 {
        return start;
    }
    array.swap(start, end - 1);
    let mut lgt_i = start;
    for i in lgt_i..end - 1 {
        if array[i] < array[end - 1] {
            array.swap(lgt_i, i);
            lgt_i += 1;
        }
    }
    array.swap(lgt_i, end - 1);
    return lgt_i;
}

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]) {
    //TODO
    let mut q = vec![];
    q.push((0 as usize, array.len()));
    loop {
        let range = q.pop();
        match range {
            Some((x, y)) => {
                let mid = partition2(array, x, y);
                if x < mid {
                    q.push((x, mid));
                }
                if mid + 1 < y {
                    q.push((mid + 1, y));
                }
            }
            None => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        let ln = vec.len();
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
