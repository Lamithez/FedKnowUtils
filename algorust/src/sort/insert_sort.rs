pub fn insert_sort(input: &Vec<i32>) -> Vec<i32> {
    let mut data = input.clone();
    for i in 1..data.len() {
        if data[i] < data[i - 1] {
            let mut p = i;
            let ele = data[i];
            while p > 0 {
                if data[p - 1] > ele {
                    data[p] = data[p - 1];
                    p -= 1;
                } else { break; }
            }
            data[p] = ele;
        }
    }
    data
}

#[cfg(test)]
mod insert_sort_test {
    use crate::sort::template::template;

    #[test]
    fn insert_sort() {
        assert!(template(crate::sort::insert_sort::insert_sort));
    }
}