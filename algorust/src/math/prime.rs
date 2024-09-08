pub fn ordinary(n: u64) -> bool {
    if n == 2 { return true; };
    if n % 2 == 0 { return false; };
    for i in 3..f64::sqrt(n as f64) as u64 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn count_primes(n: i32) -> i32 {
    table(n as u64).len() as i32
}

pub fn table(n: u64) -> Vec<u64> {
    if n <= 2 { return Vec::new(); }
    let mut prime_table = vec![2];
    for i in 3..n {
        let mut is_prime = true;
        for prime in &prime_table {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        };
        if is_prime { prime_table.push(i); }
    }
    prime_table
}


pub fn eratosthenes(n: u64) -> u64 {
    let mut table = vec![true; n as usize];
    let mut answer = 0;
    for i in 2..n {
        if table[i as usize] {
            answer += 1;
            let mut j = i;
            while (i * j < n)
            {
                table[(i * j) as usize] = false;
                j += 1;
            }
        }
    }
    answer
}


#[cfg(test)]
mod prime_test {}