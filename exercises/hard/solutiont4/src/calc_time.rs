use std::{
    collections::{BTreeMap},
};
// 闰年366天, 平年365天
fn is_leap_year(year: i32) -> bool {
    if (year % 4 == 0) && (year % 100 != 0) {
        return true;
    }
    if year % 400 == 0 {
        return true;
    }
    false
}

fn calc_weekday(year: i32, month: i32, day: i32) -> u8 {
    let (d, m, y) = if month == 1 || month == 2 {
        (day as i32, month + 12, year - 1)
    } else {
        (day as i32, month, year)
    };

    let w = (d + 2 * m + 3 * (m + 1) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
    let w = (w + 1) % 7;
    if w == 0 {
        return 7;
    }
    w as u8
}

fn calc_n_to_chinese_new_year(
    year: i32,
    month: i32,
    day: i32,
    month_days: &BTreeMap<i32, i32>,
) -> i32 {
    // 农历计算太复杂了, 此处简单处理
    // 2025年春节 2025.1.29
    // 2026年春节 2026.2.17
    if year == 2025 && month == 1 && day <= 29 {
        29 - day
    } else if year < 2026 {
        let mut total_days = 0;
        for m in month_days.iter() {
            if &month == m.0 {
                total_days += m.1 - day;
                continue;
            }
            if &month < m.0 {
                total_days += m.1;
            }
        }
        total_days += 31 + 17;
        total_days
    } else {
        0
    }
}

// A股休市安排： https://www.sse.com.cn/disclosure/dealinstruc/closed/c/c_20241223_10767110.shtml
fn calc_n_to_a_open(
    year: i32,
    month: i32,
    day: i32,
    month_days: &BTreeMap<i32, i32>,
    holidays: &Vec<(i32, i32, i32)>,
) -> i32 {
    let mut month = month;
    let start_month = month;
    let mut day = day;
    let start_day = day;
    let the_week = calc_weekday(year, month, day);

    let mut gap = 0;
    let mut today_is_holiday = false;
    let mut nextday_is_holiday = false;
    let mut should_check_next_day = false;
    let today_is_weekend = if the_week == 6 || the_week == 7 {
        true
    } else {
        false
    };
    // 当天不是节假日,则判断+1天
    if !holidays.contains(&(year, month, day)) {
        if month_days.get(&month).unwrap() < &(day + 1) {
            month = month % 12 + 1;
            day = 1;
        } else {
            day += 1;
        }
        should_check_next_day = true;
    } else {
        today_is_holiday = true;
    }

    // 计算假期还有几天结束
    while holidays.contains(&(year, month, day)) {
        if start_month != month || start_day != day {
            // 当天不计算在内
            gap += 1;
        }
        if should_check_next_day {
            nextday_is_holiday = true;
        }

        if month_days.get(&month).unwrap() < &(day + 1) {
            month = month % 12 + 1;
            day = 1;
        } else {
            day += 1;
        }
    }

    // 有节假日
    if today_is_holiday || nextday_is_holiday {
        return gap;
    }

    // 跨越周末
    if today_is_weekend || the_week == 5 {
        return 7 - the_week as i32;
    }
    // 普通工作日
    gap
}

pub fn time_info(time: &str) -> String {
    let s: Vec<&str> = time.splitn(3, '-').collect();
    let year = s[0].parse::<i32>().unwrap();
    let month = s[1].parse::<i32>().unwrap();
    let day = s[2].parse::<i32>().unwrap();
    let mut month_days = BTreeMap::new();
    month_days.insert(1, 31);
    month_days.insert(3, 31);
    month_days.insert(5, 31);
    month_days.insert(7, 31);
    month_days.insert(8, 31);
    month_days.insert(10, 31);
    month_days.insert(12, 31);
    month_days.insert(4, 30);
    month_days.insert(6, 30);
    month_days.insert(9, 30);
    month_days.insert(11, 30);
    let year_days = if is_leap_year(year) {
        month_days.insert(2, 29);
        366
    } else {
        month_days.insert(2, 28);
        365
    };

    // 本年已过几天
    let pass_days = if month == 1 {
        day
    } else {
        let mut total_days = 0;
        for m in month_days.iter() {
            if m.0 < &month {
                total_days += m.1;
            }
        }
        total_days += day;
        total_days
    };

    // 本年第几天
    let the_n_day_on_year = if pass_days % year_days == 0 {
        pass_days
    } else {
        pass_days % year_days
    };

    // 本年还剩余几天
    let the_rest_days = year_days - pass_days;

    // 第几周
    let mut t_week = (pass_days as f32 / 7.0).ceil() as i32;
    if pass_days % 7 == 0 {
        t_week += 1;
    }
    let the_n_week_on_year = t_week % 52;
    // 周几
    let the_week = calc_weekday(year, month, day);

    // 距离春节
    let the_n_day_to_cny = calc_n_to_chinese_new_year(year, month, day, &month_days);

    // 距离A股开盘
    let closed_on_holiday = vec![
        (2025, 1, 1),
        (2025, 1, 28),
        (2025, 1, 29),
        (2025, 1, 30),
        (2025, 1, 31),
        (2025, 2, 1),
        (2025, 2, 2),
        (2025, 2, 3),
        (2025, 2, 4),
        (2025, 4, 4),
        (2025, 4, 5),
        (2025, 4, 6),
        (2025, 5, 1),
        (2025, 5, 2),
        (2025, 5, 3),
        (2025, 5, 4),
        (2025, 5, 5),
        (2025, 5, 31),
        (2025, 6, 1),
        (2025, 6, 2),
        (2025, 10, 1),
        (2025, 10, 2),
        (2025, 10, 3),
        (2025, 10, 4),
        (2025, 10, 5),
        (2025, 10, 6),
        (2025, 10, 7),
        (2025, 10, 8),
        (2026, 1, 1),
    ];
    let the_n_day_to_a_open = calc_n_to_a_open(year, month, day, &month_days, &closed_on_holiday);

    let result = format!(
        "{},{},{},{},{},{}",
        the_n_week_on_year,
        the_week,
        the_n_day_on_year,
        the_rest_days,
        the_n_day_to_cny,
        the_n_day_to_a_open
    );
    // println!("本年第{}周", the_n_week_on_year);
    // println!("星期{}", the_week);
    // println!("本年第{}天", the_n_day_on_year);
    // println!("本年剩余{}天", the_rest_days);
    // println!("距离春节{}天", the_n_day_to_cny);
    // println!("距离开盘{}天", the_n_day_to_a_open);
    // println!("####{}", result);
    result
}