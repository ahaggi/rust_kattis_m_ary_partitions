fn main() {
    // if n > m
    // let m = 3;
    // let n = 47;

    // let mut m_pow = m;
    // let mut _n = n;
    // let mut res = 0;

    // if m >= n {
    //     res = 1;
    // } else {
    //     while m_pow < n {
    //         res += (_n / m) + 1;

    //         m_pow *= m;
    //         _n = n % m_pow;
    //         println!("{}", res);

    //     }
    //     if n % m == 0 {
    //         res += 1;
    //     }
    // }

    println!("{}", wer(2 , 3, 27));
}

fn wer(qwe: u32, m: u32, num: u32) -> u32 {
    let res: u32 = 0;
    if qwe == 0 {
        return 1;
    };

    if qwe == 1 {
        return num / m;
    };

    if qwe == 2 {
        return s_i(num / m, m);
    }

    0
}
fn ge_cube(qwe: u32, m: u32, num: u32) -> u32 {
    //for numbers that are bigger or  equel to m*m*m or cube_of_m
    let mut res = 1;
    println!("{} {} {}", qwe, m, num);

    if num == m * m * m {
        res = s_i(qwe, m);
    } else {
        for _i in 1..m + 1 {
            res += ge_cube(_i * m + 1, m, num / m);
        }
    }
    println!("{} {} {} res = {}", qwe, m, num, res);
    res
}

fn s_i(i: u32, m: u32) -> u32 {
    let a_i = ((i - 1) * m) + 1;

    let s_i = i * (1 + a_i) / 2;

    s_i
}
