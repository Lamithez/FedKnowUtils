use crate::sort::template::template;

pub fn select_sort(input: &Vec<i32>) -> Vec<i32> {
    let mut data = input.clone();
    for i in 0..data.len() {
        let mut min = i;
        for j in i + 1..data.len() {
            if data[j] < data[min] {
                min = j;
            }
        }
        data.swap(i, min);
    }
    data
}


#[test]
fn select_sort_test() {
    assert!(template(select_sort));
}