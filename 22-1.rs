use std::io::stdin;

fn nextval(mut n: u64) -> u64
{
    n ^= n << 6;
    n &= 16777215;
    n ^= n >> 5;
    n ^= n << 11;
    return n & 16777215;
}

fn nextvals(mut n: u64, cnt: u64) -> u64
{
    for _ in 0..cnt
    {
        n = nextval(n);
    }
    return n;
}

fn main_rs(_args: Vec<String>) -> i32
{
    let nums: Vec<u64> = stdin().lines().map(|x|x.unwrap().parse().unwrap()).collect();
    println!("{}", nums.iter().map(|x|nextvals(*x, 2000)).sum::<u64>());
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
