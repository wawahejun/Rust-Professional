pub fn goldbach_conjecture() -> String {
    let mut primes = vec![true; 10000];
    primes[0] = false;
    primes[1] = false;

    // 使用埃拉托色尼筛法生成素数列表
    for i in 2..10000 {
        if primes[i] {
            let mut multiple = i * 2;
            while multiple < 10000 {
                primes[multiple] = false;
                multiple += i;
            }
        }
    }

    let mut invalid_numbers = vec![];

    // 遍历奇合数
    for i in (9..10000).step_by(2) {
        if !primes[i] {
            let mut found = false;

            // 检查是否可以表示为一个素数和一个平方的两倍之和
            for p in 2..=i {
                if primes[p] {
                    let mut n = 0;
                    while p + 2 * n * n <= i {
                        if p + 2 * n * n == i {
                            found = true;
                            break;
                        }
                        n += 1;
                    }
                }
                if found {
                    break;
                }
            }

            // 如果不能表示为一个素数和一个平方的两倍之和
            if !found {
                invalid_numbers.push(i);
                if invalid_numbers.len() == 2 {
                    break;
                }
            }
        }
    }

    // 返回结果
    format!("{},{}", invalid_numbers[0], invalid_numbers[1])
}