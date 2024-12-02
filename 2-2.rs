use std::io;
use std::mem::swap;

fn check(arr: &Vec<i32>) -> bool
{
    let (mut f, mut b) = (0, 0);
    for s in arr.windows(2)
    {
        let (u, v) = (s[0], s[1]);
        if !(u < v && v - u >= 1 && v - u <= 3)
        {
            f += 1;
        }
    }
    for s in arr.windows(2).rev()
    {
        let (u, v) = (s[1], s[0]);
        if !(u < v && v - u >= 1 && v - u <= 3)
        {
            b += 1;
        }
    }
    return f == 0 || b == 0;
}

fn checkall(arr: &mut Vec<i32>) -> bool
{
    if check(arr)
    {
        return true;
    }
    let mut curr = arr.pop().unwrap();
    for i in 0..arr.len()
    {
        let ind = arr.len() - i - 1;
        if check(arr)
        {
            return true;
        }
        swap(&mut arr[ind], &mut curr);
    }
    return check(arr);
}

fn main()
{
    let mut ln = String::new();
    let mut bc = io::stdin().read_line(&mut ln).unwrap();
    let mut cnt = 0;
    while bc > 0
    {
        ln.pop();
        let mut arr: Vec<i32> = ln.split(" ").map(|str|str.parse().unwrap()).collect();
        if checkall(&mut arr)
        {
            cnt += 1;
        }
        ln.clear();
        bc = io::stdin().read_line(&mut ln).unwrap();
    }
    println!("{}", cnt);
}
