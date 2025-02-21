pub fn dp_rec_mc(amount: u32) -> u32 {
    // 定义币种数组
    let denominations = [1, 2, 5, 10, 20, 30, 50, 100];
    // 初始化一个数组，用于存储每个金额的最小纸币数
    let mut dp = vec![u32::MAX; amount as usize + 1];
    dp[0] = 0; // 金额为0时，不需要纸币

    // 遍历每个金额
    for i in 1..=amount as usize {
        for &coin in &denominations {
            if coin <= i as u32 {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }
    }

    if dp[amount as usize] == u32::MAX {
        0 // 如果无法凑成该金额，返回0
    } else {
        dp[amount as usize]
    }
}