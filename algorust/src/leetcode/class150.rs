pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut p1 = m - 1;
    let mut p2 = n - 1;
    let mut p = p1 + p2 + 1;
    while p1 >= 0 && p2 >= 0 {
        if nums1[p1 as usize] > nums2[p2 as usize] {
            nums1[p as usize] = nums1[p1 as usize];
            p1 -= 1;
        } else {
            nums1[p as usize] = nums2[p2 as usize];
            p2 -= 1;
        }
        p -= 1;
    }
    if (p2 >= 0) {
        for (index, data) in nums2[0..=p2 as usize].iter().enumerate() {
            nums1[index] = *data;
        }
    }
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[index] = nums[i];
            index += 1;
        }
    }
    index as i32
}


pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut p = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[p - 1] {
            nums[p] = nums[i];
            p += 1;
        }
    }
    p as _
}

pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
    let mut p = 2;
    for i in 2..nums.len() {
        if nums[i] != nums[p - 2] {
            nums[p] = nums[i];
            p += 1;
        }
    }
    p.min(nums.len()) as _
}


pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut candidate, mut count) = (0, 0);
    nums.iter().for_each(|&num| {
        if 0 == count { candidate = num; }
        count += if candidate == num { 1 } else { -1 }
    });
    candidate
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    nums.rotate_right(k);
}


pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut lowerst = prices[0];
    let mut max_profit = 0;
    for index in 1..prices.len() {
        if prices[index] < lowerst {
            lowerst = prices[index];
        } else if prices[index] - lowerst > max_profit {
            max_profit = prices[index] - lowerst;
        }
    }
    max_profit
}
pub fn max_profit_2(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            profit += prices[i] - prices[i - 1];
        }
    }
    profit
}


pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut farest = 0;
    for i in 0..nums.len() {
        if farest < i {
            return false;
        }
        farest = farest.max(i + nums[i] as usize);
    }
    true
}


pub fn jump(nums: Vec<i32>) -> i32 {
    if nums[0] == 0 { return 0; }
    let mut farst = 0;
    let mut current = 0;
    let mut step = 0;
    loop {
        for i in current..=farst {
            farst = farst.max(i + nums[i] as usize);
            if farst >= nums.len() - 1 {
                return step + 1;
            }
        }
        step += 1;
    }
}


pub fn h_index(citations: Vec<i32>) -> i32 {
    let total = citations.len();
    let mut counter = vec![0; total + 1];
    for cite in citations {
        if cite >= total as i32 {
            counter[total] += 1;
        } else {
            counter[cite as usize] += 1;
        }
    }
    let mut tot = 0;
    for i in (0..=total).rev() {
        tot += counter[i];
        if tot >= i {
            return i as i32;
        }
    }
    0
}


pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let size = nums.len();

    let mut result = nums.iter()
        .scan(1, |acc, x| {
            let current = *acc;
            *acc *= x;
            Some(current)
        })
        .collect::<Vec<_>>();

    let mut left_base = 1;

    for i in (0..size).rev() {
        result[i] *= left_base;
        left_base *= nums[i];
    }
    result
}


pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let station_count = gas.len() as i32;
    let mut start = 0;
    while start < station_count {
        let mut remain_gas = 0;
        let mut step = 0;
        while step < station_count {
            let post = ((step + start) % station_count) as usize;
            remain_gas += gas[post] - cost[post];
            if remain_gas < 0 {
                break;
            }
            step += 1;
        }
        if step < station_count { start += step + 1; } else { return start; }
    }
    -1
}


pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let (mut total, mut current) = (1, 1);
    let (mut decrease_length, mut increase_height) = (0, 1);
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            decrease_length = 0;
            current = current + 1;
            total += current;
            increase_height = current;
        } else if ratings[i] == ratings[i - 1] {
            decrease_length = 0;
            (current, increase_height) = (1, 1);
            total += 1;
        } else {
            decrease_length += 1;
            if (decrease_length == increase_height) {
                decrease_length += 1;
            }
            total += decrease_length;
            current = 1;
        }
    }
    total
}