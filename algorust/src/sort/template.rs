use rand::{Rng, thread_rng};

pub fn template<T: Fn(&Vec<i32>) -> Vec<i32>>(func: T) -> bool {
    let data = generate_data();
    for mut vec in data {
        let answer = func(&vec);
        vec.sort();
        if answer != vec {
            assert_eq!(answer,vec);
            return false; }
    }
    true
}


fn generate_data() -> Vec<Vec<i32>> {
    let mut ret = Vec::new();
    let mut random_handle = thread_rng();
    let mount = random_handle.gen_range(0..50);
    for _ in 0..mount {
        let mount = random_handle.gen_range(0..200);
        let mut data = Vec::with_capacity(mount);
        for _ in 0..mount {
            data.push(random_handle.gen_range(-500..500));
        }
        ret.push(data);
    }
    ret
}


// #[test]
// fn data_gen() {
//     let data = generate_data();
//     assert_eq!(data[0], data[1]);
// }
