pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold == 0 {
        return 0;
    }
    if threshold == 1 {
        return 1;
    }

    let mut prev: u32 = 0;
    let mut curr: u32 = 1;
    let mut sum = 1u32;
    loop {
        let next = prev + curr;
        if next > threshold {
            break;
        }
        if next % 2 == 1 {
            sum += next;
        }
        prev = curr;
        curr = next;
    }
    sum
}
