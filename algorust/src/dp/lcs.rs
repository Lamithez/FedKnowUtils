pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let s1 = text1.as_bytes();
    let s2 = text2.as_bytes();
    let l1 = text1.len();
    let l2 = text2.len();
    let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
    for i in 1..=l1 {
        let c1 = s1[i - 1];
        for j in 1..=l2 {
            let c2 = s2[j - 1];
            if c1 == c2 {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[l1][l2]
}

