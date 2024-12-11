use std::collections::LinkedList;
use std::io::stdin;

fn transform(list:&mut LinkedList<(u64, u64)>, stone: (u64, u64))
{
    if stone.1 == 0
    {
        list.push_back((stone.0 + 1, 1));
    }
    else if stone.1.ilog10() % 2 == 1
    {
        let split = stone.1.ilog10();
        let pow = 10u64.pow((split + 1) / 2);
        list.push_back((stone.0 + 1, stone.1 / pow));
        list.push_back((stone.0 + 1, stone.1 % pow));
    }
    else
    {
        list.push_back((stone.0 + 1, stone.1 * 2024));
    }
}

fn main()
{
    let mut ln = String::new();
    let _bc = stdin().read_line(&mut ln);
    ln.pop();
    let mut q: LinkedList<(u64, u64)> = ln.split(" ").map(|str|(0, str.parse().unwrap())).collect();
    while q.front().unwrap().0 < 25
    {
        let curr = q.pop_front().unwrap();
        transform(&mut q, curr);
    }
    println!("{}", q.len());
}
