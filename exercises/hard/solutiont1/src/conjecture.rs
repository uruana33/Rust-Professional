fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in bases.iter() {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                break;
            }
        }
        if x != n - 1 {
            return false;
        }
    }
    true
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = do_mod(result, base, modulus);
        }
        exp >>= 1;
        base = do_mod(base, base, modulus);
    }
    result
}

fn do_mod(a: u128, b: u128, m: u128) -> u128 {
    let mut a = a % m;
    let mut b = b % m;
    let mut res = 0;
    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % m;
        }
        a = (a * 2) % m;
        b /= 2;
    }
    res
}

pub fn goldbach_conjecture() -> String {
    let mut prims = vec![2];
    let mut found = vec![];
    for num in 3.. {
        if found.len() == 2 {
            break;
        }
        if num % 2 == 0 {
            continue;
        }
        if is_prime(num) {
            prims.push(num);
            continue;
        }

        let mut is_ok = false;
        for n in prims.iter().rev() {
            if num == *n {
                continue;
            }
            let t = (num - n) / 2;
            let i = t.isqrt();
            if i * i == t {
                is_ok = true;
                break;
            }
        }
        if !is_ok {
            found.push(num);
        }
    }
    format!("{},{}", found[0], found[1])
}