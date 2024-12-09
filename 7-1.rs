use std::io::stdin;

fn check(target: u64, arr: &Vec<u64>) -> bool
{
    let end = 1u64 << (arr.len() - 1);
    for i in 0..end
    {
        let mut acc: u64 = arr[0];
        for (j, v) in arr.iter().skip(1).enumerate()
        {
            if (i >> j & 1) == 1
            {
                acc *= *v;
            }
            else
            {
                acc += *v;
            }
            if acc > target {break;}
        }
        if acc == target{return true;}
    }
    return false;
}

fn main()
{
    let mut cnt = 0;
    let mut line = String::new();
    let mut bcnt = stdin().read_line(&mut line).unwrap();
    while bcnt > 0
    {
        line.pop();
        let mut it = line.split(": ");
        let v: u64 = it.next().unwrap().parse().unwrap();
        let nums = it.next().unwrap().split(" ").map(|s|s.parse().unwrap()).collect();
        if check(v, &nums)
        {
            cnt += v;
        }
        line.clear();
        bcnt = stdin().read_line(&mut line).unwrap();
    }
    println!("{}", cnt);
}
