use std::collections::HashMap;
use std::io::stdin;

fn nextval(mut n: u64) -> u64
{
    n ^= n << 6;
    n &= 16777215;
    n ^= n >> 5;
    n ^= n << 11;
    return n & 16777215;
}

fn nextvals(mut n: u64, cnt: u64) -> Vec<[u64; 2]>
{
    let mut v: Vec<[u64; 2]> = Vec::new();
    for _ in 0..cnt
    {
        let m = nextval(n);
        v.push([m % 10, m % 10 - n % 10]);
        n = m;
    }
    return v;
}

fn main_rs(_args: Vec<String>) -> i32
{
    let nums: Vec<u64> = stdin().lines().map(|x|x.unwrap().parse().unwrap()).collect();
    let mut slices: HashMap<[u64; 4], u64> = HashMap::new();
    for i in nums.iter()
    {
        let mut smallslices: HashMap<[u64; 4], u64> = HashMap::new();
        let seq = nextvals(*i, 2000);
        for j in seq.windows(4)
        {
            let _ = *smallslices.entry([j[0][1], j[1][1], j[2][1], j[3][1]]).or_insert(j[3][0]);
        }
        for (k, v) in smallslices
        {
            *slices.entry(k).or_insert(0) += v;
        }
    }
    println!("{}", slices.values().max().unwrap());
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
