// 素因数分解
pub fn prime_factorization(n: u64) -> std::collections::BTreeMap<u64, u64> {
    if let Ok(output) = std::process::Command::new("factor")
        .arg(n.to_string())
        .output()
    {
        let mut res = std::collections::BTreeMap::new();
        if n == 1 {
            return res;
        }

        let output = String::from_utf8(output.stdout)
            .unwrap()
            .trim_start_matches(&format!("{}: ", n))
            .trim()
            .to_string();

        for s in output.split(" ") {
            *res.entry(s.parse::<u64>().unwrap()).or_insert(0) += 1;
        }
        res
    } else {
        /* n = 1e12ぐらいまでじゃないとTLEするので注意 */
        let mut res = std::collections::BTreeMap::new();

        let mut i = 2;
        let mut k = n;
        while i * i <= n {
            let mut cnt = 0;
            while k % i == 0 {
                k /= i;
                cnt += 1;
            }
            if cnt != 0 {
                res.insert(i, cnt);
            }
            i += 1;
        }

        if k != 1 {
            res.insert(k, 1);
        }
        res
    }
}

/* nまでの素数を生成して配列で返す */
pub fn create_primes(n: usize) -> Vec<u64> {
    let mut res = vec![];
    let mut ck = vec![false; n];

    let mut i = 2;
    while i < n {
        if !ck[i] {
            res.push(i as u64);
            let mut j = i;
            while j < n {
                ck[j] = true;
                j += i;
            }
        }
        i += 1;
    }

    res
}

// 約数全列挙
pub fn divisors(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
        i += 1;
    }
    res
}
