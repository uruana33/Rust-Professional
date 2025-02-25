pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let s = num_str.replace("(", " ").replace(")", "");
    let mut s = s.split_whitespace();
    let num = s.next().unwrap();
    let from_base = s.next().unwrap().parse::<i32>().unwrap();

    let mut n = 0;
    let mut tmp = 0;
    for i in num.chars().rev() {
        let int = i.to_string().parse::<u32>().unwrap();
        tmp += int * ((from_base.pow(n)) as u32);
        n += 1;
    }

    let mut v = String::new();
    if tmp == to_base {
        return "10".to_string();
    }

    fn covert(v: u32) -> String {
        match v {
            r @ 0..=9 => r.to_string(),
            10 => "a".into(),
            11 => "b".into(),
            12 => "c".into(),
            13 => "d".into(),
            14 => "e".into(),
            15 => "f".into(),
            _ => unreachable!(),
        }
    }
    while tmp / to_base != 0 {
        (&mut v).push_str(&covert(tmp / to_base));
        tmp /= to_base;
    }
    v.push_str(&covert(tmp % to_base));
    v
}
