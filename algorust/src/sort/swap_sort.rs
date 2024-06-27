pub fn bible_sort(data: &Vec<i32>) -> Vec<i32> {
    let mut data = data.clone();
    for i in 1..data.len() {
        for j in 0..data.len() - i {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
            }
        }
    }
    data
}


fn partition(data: &mut Vec<i32>, start: usize, end: usize) -> usize {
    let mut left = start;
    let mut right = end;
    let base = data[left];
    while left < right {
        while left < right && data[right] >= base {
            right -= 1;
        }
        data[left] = data[right];
        while left < right && data[left] <= base {
            left += 1;
        }
        data[right] = data[left]
    }
    data[left] = base;
    left
}

fn quick_sort_recursion(data: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
        let center = partition(data, start, end);
        if center >= 1 {
            quick_sort_recursion(data, start, center - 1);
        }
        quick_sort_recursion(data, center + 1, end);
    }
}

pub fn quick_sort(data: &Vec<i32>) -> Vec<i32> {
    let mut data = data.clone();
    if data.is_empty() {
        return data;
    }
    let end = data.len() - 1;
    quick_sort_recursion(&mut data, 0, end);
    return data;
}

#[cfg(test)]
mod test {
    use crate::sort::swap_sort::{bible_sort, quick_sort};
    use crate::sort::template::template;

    #[test]
    fn bible_test() {
        assert!(template(bible_sort));
    }

    #[test]
    fn quick_test() {
        assert!(template(quick_sort));
    }
}