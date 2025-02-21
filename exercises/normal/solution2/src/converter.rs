pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 解析输入字符串，提取原始数字和原始进制
    let (num_part, from_base) = if let Some(index) = num_str.find('(') {
        let num_part = &num_str[0..index];
        let base_str = &num_str[index + 1..num_str.len() - 1];
        let from_base = base_str.parse::<u32>().unwrap();
        (num_part, from_base)
    } else {
        (num_str, 10)
    };

    // 将原始数字转换为十进制
    let decimal_num = u32::from_str_radix(num_part, from_base).unwrap();

    // 定义字符集，用于目标进制的转换
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
        'a', 'b', 'c', 'd', 'e', 'f'
    ];

    let mut result = String::new();
    let mut quotient = decimal_num;

    if quotient == 0 {
        result.push('0');
    } else {
        while quotient > 0 {
            let remainder = quotient % to_base;
            result.push(digits[remainder as usize]);
            quotient /= to_base;
        }
        result = result.chars().rev().collect::<String>();
    }

    result
}