use crate::sort::template::template;

const BUCKET_SIZE: usize = 10;

pub fn bucket_sort(input: &Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return Vec::new();
    }
    let (max, min) = (input.iter().max().unwrap(), input.iter().min().unwrap());
    let bucket_count: usize = (max - min) as usize / BUCKET_SIZE + 1;

    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); bucket_count];
    for val in input {
        let index = (val - min) as usize / BUCKET_SIZE;
        buckets[index].push(*val);
    }
    let mut data = Vec::with_capacity(input.len());
    for bucket in &mut buckets {
        bucket.sort();
        data.append(bucket);
    }
    data
}


fn radix_sort_positive(mut data: &mut Vec<i32>) {
    if data.len() <= 1 { return; }
    let max_num = *data.iter().max().unwrap();
    let mut max_level = 1;
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); 10];
    while max_level < max_num {
        for i in data.iter() {
            let index = *i / max_level % 10;
            buckets[index as usize].push(*i);
        }
        *data = vec![];
        for bucket in buckets.iter_mut() {
            data.append(bucket);
        }
        max_level *= 10;
    }
}

pub fn radix_sort(input: &Vec<i32>) -> Vec<i32> {
    let mut positive = Vec::new();
    let mut negtive = Vec::new();
    for i in input {
        if *i >= 0 {
            positive.push(*i);
        } else {
            negtive.push(-*i);
        }
    }
    radix_sort_positive(&mut positive);
    radix_sort_positive(&mut negtive);
    let mut ret = negtive.iter().map(|x| -x).collect::<Vec<i32>>();
    ret.reverse();
    ret.append(&mut positive);
    ret
}

#[test]
fn bucket_sort_test() {
    assert!(template(bucket_sort));
}

#[test]
fn radix_sort_test(){
    assert!(template(radix_sort));
}