pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut sum = 0;
    let mut a = 0;
    let mut b = 1;

    while b <= threshold {
        if b % 2 != 0 {
            sum += b;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    sum
}