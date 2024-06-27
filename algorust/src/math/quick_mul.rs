use rand::{Rng, thread_rng};

pub fn quick_mul(x: f64, mut n: i32) -> f64 {
    let mut answer = 1.0;
    let mut level = x;
    while n != 0 {
        if n & 1 == 1 {
            answer *= level;
        }
        level *= x;
        n >>= 1;
    }
    answer
}

//跑不过
#[test]
fn quick_mul_test() {
    let mut rng = thread_rng();
    for _ in 0..20 {
        let x = rng.gen_range(0.0..100000.0);
        let n = rng.gen_range(0..1000);
        let ret = quick_mul(x, n);
        println!("{}^{}", x, n);
        assert_eq!(ret, x.powi(n));
    }
}