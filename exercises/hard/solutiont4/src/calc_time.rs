// 定义常量避免魔法数字
const WEEKEND_SUNDAY: i32 = 7;
const WEEKEND_SATURDAY: i32 = 6;
const WEEKEND_FRIDAY: i32 = 5;
const WEEKEND_MONDAY: i32 = 1;

// 使用 Zeller 公式计算星期几
fn calculate_weekday(year: i32, month: i32, day: i32) -> i32 {
    let mut y = year;
    let mut m = month;
    let q = day;

    // 1月和2月被视为前一年的13月和14月
    if m < 3 {
        m += 12;
        y -= 1;
    }

    let k = y % 100;   // 年份的后两位
    let j = y / 100;   // 年份的前两位

    // Zeller 公式的简化形式
    let h = (q + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7;
    let res = (h + 5) % 7;
    
    // 返回 1-7 表示周一至周日
    if res == 0 {
        return 7;  // 周日
    } else {
        res + 1    // 周一至周六
    }
}

// 判断是否为闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// 计算从年初到指定日期的天数
fn days_of_year(year: i32, month: i32, day: i32) -> i32 {
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day_count = 0;

    for m in 1..month {
        day_count += days_in_month[m as usize - 1];
    }

    // 闰年2月后的日期需要加1天
    if month > 2 && is_leap_year(year) {
        day_count += 1;
    }

    day_count + day
}

// 计算一年中的第几周
fn week_of_year(year: i32, month: i32, day: i32) -> i32 {
    let day_of_year = days_of_year(year, month, day);
    (day_of_year + 1) / 7 + 1
}

// 判断是否为法定节假日
fn is_holiday(year: i32, month: i32, day: i32) -> bool {
    // 2025年的法定节假日列表
    let holidays = vec![
        // 春节假期
        (2025, 1, 28), (2025, 1, 29), (2025, 1, 30), (2025, 1, 31), 
        (2025, 2, 1), (2025, 2, 2), (2025, 2, 3), (2025, 2, 4),
        // 国庆假期
        (2025, 10, 1), (2025, 10, 2), (2025, 10, 3), (2025, 10, 4), 
        (2025, 10, 5), (2025, 10, 6), (2025, 10, 7),
    ];
    holidays.contains(&(year, month, day))
}

// 判断是否为交易日（非周末且非法定节假日）
fn is_trading_day(year: i32, month: i32, day: i32) -> bool {
    let weekday = calculate_weekday(year, month, day);
    !is_holiday(year, month, day) && weekday != WEEKEND_MONDAY && weekday != WEEKEND_SUNDAY
}

// 计算下一天的日期
fn add_one_day(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut new_day = day + 1;
    let mut new_month = month;
    let mut new_year = year;

    // 处理月末情况，考虑闰年2月
    let month_days = days_in_month[month as usize - 1] 
        + if month == 2 && is_leap_year(year) { 1 } else { 0 };
    
    if new_day > month_days {
        new_day = 1;
        new_month += 1;

        // 处理年末情况
        if new_month > 12 {
            new_month = 1;
            new_year += 1;
        }
    }

    (new_year, new_month, new_day)
}

// 计算距离下一个A股开市的天数
fn days_to_next_a_share_opening(year: i32, month: i32, day: i32) -> i32 {
    // 各个假期后的开市日
    let new_year_day_open = days_of_year(year, 1, 2);           // 元旦后
    let spring_year_day_open = days_of_year(year, 2, 5);        // 春节后
    let qingming_day_open = days_of_year(year, 4, 7);           // 清明后
    let labor_day_open = days_of_year(year, 5, 6);              // 劳动节后
    let zongzi_day_open = days_of_year(year, 6, 3);             // 端午后
    let autumn_day_open = days_of_year(year, 10, 9);            // 国庆后
    let next_new_year_day_open = days_of_year(year + 1, 1, 1);  // 下一年元旦后

    // 当前日期的天数
    let current_day_of_year = days_of_year(year, month, day);
    
    // 处理各个假期期间
    if month == 1 && day == 1 {
        return new_year_day_open - current_day_of_year - 1;
    }
    if (month == 1 && (28..=31).contains(&day)) || (month == 2 && (1..=4).contains(&day)) {
        return spring_year_day_open - current_day_of_year - 1;
    }
    if month == 4 && (4..=6).contains(&day) {
        return qingming_day_open - current_day_of_year - 1;
    }
    if month == 5 && (1..=5).contains(&day) {
        return labor_day_open - current_day_of_year - 1;
    }
    if (month == 5 && day == 31) || (month == 6 && (1..=2).contains(&day)) {
        return zongzi_day_open - current_day_of_year - 1;
    }
    if month == 10 && (1..=8).contains(&day) {
        return autumn_day_open - current_day_of_year - 1;
    }
    if month == 12 && day == 31 {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        return next_new_year_day_open - current_day_of_year + days_in_year;
    }

    // 处理周末情况
    let weekday = calculate_weekday(year, month, day);
    match weekday {
        WEEKEND_SUNDAY => 0,  // 周日
        WEEKEND_FRIDAY => 2,  // 周五，距离下周一开盘 2 天
        WEEKEND_SATURDAY => 1,  // 周六，距离周一开盘 1 天
        _ => 0,  // 周一到周四，如果是交易日则为0
    }
}

// 查找下一个交易日
fn next_trading_day(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    let mut current_year = year;
    let mut current_month = month;
    let mut current_day = day;

    // 如果当天不是交易日，寻找下一个交易日
    while !is_trading_day(current_year, current_month, current_day) {
        let (new_year, new_month, new_day) = add_one_day(current_year, current_month, current_day);
        current_year = new_year;
        current_month = new_month;
        current_day = new_day;
    }

    (current_year, current_month, current_day)
}

// 主函数：根据日期计算各种信息
pub fn time_info(time: &str) -> String {
    // 解析输入的日期字符串
    let times: Vec<&str> = time.split("-").collect();
    let year: i32 = times[0].parse().unwrap_or_default();
    let month: i32 = times[1].parse().unwrap_or_default();
    let day: i32 = times[2].parse().unwrap_or_default();
    
    // 计算各种日期信息
    let mut week_of_year = week_of_year(year, month, day);
    let day_of_week = calculate_weekday(year, month, day);
    let day_of_year = days_of_year(year, month, day);
    
    // 计算到年底还剩多少天
    let day_Rest = if is_leap_year(year) {
        366 - day_of_year
    } else {
        365 - day_of_year
    };
    
    // 计算到春节还剩多少天（可能是2025年春节的特定逻辑）
    let day_lunar = if day_of_year > 29 {
        413 - day_of_year
    } else {
        29 - day_of_year
    };
    
    // 查找下一个交易日
    let (_, _, _) = next_trading_day(year, month, day);
    
    // 计算到下一个交易日还有多少天
    let days_to_trading = days_to_next_a_share_opening(year, month, day);
    
    // 处理周数超过52的情况
    if week_of_year >= 53 {
        week_of_year = 1;
    }
    
    // 格式化输出结果
    format!(
        "{},{},{},{},{},{}",
        week_of_year,
        day_of_week,
        day_of_year,
        day_Rest,
        day_lunar,
        days_to_trading
    )
}