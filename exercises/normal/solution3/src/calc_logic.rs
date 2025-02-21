pub fn new_birthday_probability(n: u32) -> f64 {
    if n < 2 {
        return 0.0; // 防止输入无效的情况（虽然题目要求n >=2）
    }
    if n > 365 {
        return 1.0; // 当人数超过365人时，必定存在生日相同的情况
    }

    let mut unique_birthday_prob = 1.0; // 所有人生日互不相同的概率

    for i in 1..n {
        unique_birthday_prob *= (365.0 - i as f64) / 365.0;
    }

    let probability = 1.0 - unique_birthday_prob; // 至少两人生日相同的概率

    // 保留四位小数
    (probability * 10000.0).round() / 10000.0
}
