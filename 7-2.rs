use std::io::stdin;

fn concat(x:&mut u64, y: u64)
{
    let mut mult: u64 = 1;
    while mult <= y {mult *= 10;}
    *x *= mult;
    *x += y;
}

fn check(target: u64, arr: &Vec<u64>) -> bool
{
    let end = 1u64 << ((arr.len() - 1) * 2);
    for i in 0..end
    {
        let mut acc: u64 = arr[0];
        for (j, v) in arr.iter().skip(1).enumerate()
        {
            let op = i >> (j * 2) & 0b11;
            match op
            {
                0 => acc += *v,
                1 => acc *= *v,
                2 => concat(&mut acc, *v),
                _ => {break;}
            };
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
