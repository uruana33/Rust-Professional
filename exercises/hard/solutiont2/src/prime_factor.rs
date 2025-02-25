
pub fn find_max_prime_factor(mut number: u128) -> u128 {
    let mut max_prime = 1;
    let base_primes = vec![2, 3, 5, 7, 11, 13, 17, 19];
    for i in base_primes {
        while number % i == 0 {
            max_prime = max_prime.max(i);
            number /= i;
        }
    }
    // 如果是素数那么提前终止,因为若不是话，那么还会继续分解,数值会继续缩小
    // 性能关键
    if is_prime(number) {
        return max_prime.max(number);
    }
    let mut factor = 23;
    let step = 2; // 只处理奇数
    let mut sqrt_val = number.isqrt();
    while factor <= sqrt_val && number > 1 {
        while number % factor == 0 {
            max_prime = factor;
            number /= factor;
            sqrt_val = number.isqrt();
        }
        if is_prime(number) {
            return max_prime.max(number);
        }
        factor += step;
    }
    if number > 1 {
        max_prime = max_prime.max(number);
    }
    max_prime
}

// 确定性Miller-Rabin测试（覆盖u128范围）
// AI生成
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
