pub fn retire_time(time: &str, tp: &str) -> String {
    let birth_parts: Vec<&str> = time.split('-').collect();
    let birth_year: i32 = birth_parts[0].parse().unwrap();
    let birth_month: i32 = birth_parts[1].parse().unwrap();

    let (old_retire_age, new_retire_age, delay_months): (i32, i32, i32) = match tp {
        "男职工" => (60, 63, 4),
        "原法定退休年龄55周岁女职工" => (55, 58, 4),
        "原法定退休年龄50周岁女职工" => (50, 55, 2),
        _ => panic!("无法计算"),
    };

    let old_retire_year = birth_year + old_retire_age;
    let old_retire_month = birth_month;

    let start_year = 2025;
    let start_month = 1;

    let use_new_policy = (old_retire_year > start_year)
        || (old_retire_year == start_year && old_retire_month >= start_month);

    if !use_new_policy {
        return format!(
            "{:04}-{:02},{},0",
            old_retire_year, old_retire_month, old_retire_age
        );
    }

    let total_months_to_delay = (new_retire_age - old_retire_age) * 12;

    let months_after_policy_start =
        (old_retire_year - start_year) * 12 + (old_retire_month - start_month);

    let delay_months = if months_after_policy_start <= 0 {
        (total_months_to_delay.min(1)).max(0)
    } else {
        let progressive_delay = (months_after_policy_start + delay_months - 1) / delay_months;
        progressive_delay.min(total_months_to_delay)
    };

    let total_months = old_retire_year * 12 + old_retire_month + delay_months - 1;
    let new_retire_year = total_months / 12;
    let new_retire_month = total_months % 12 + 1;

    let retirement_age_in_months = old_retire_age * 12 + delay_months;
    let retirement_age = retirement_age_in_months as f64 / 12.0;
    let retirement_age_formatted = if retirement_age.fract() == 0.0 {
        format!("{:.0}", retirement_age)
    } else {
        format!("{:.2}", retirement_age)
    };

    format!(
        "{:04}-{:02},{},{}",
        new_retire_year, new_retire_month, retirement_age_formatted, delay_months
    )
}
