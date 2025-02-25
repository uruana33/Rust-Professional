const CASHES: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut total = 0;
    let mut money = amount;
    for cash in CASHES.iter().rev() {
        let n = money / cash;
        if n == 0 {
            continue;
        }
        total += n;
        money %= cash;
    }
    total
}
