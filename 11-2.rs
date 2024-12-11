use std::collections::HashMap;
use std::io::stdin;

fn transform(list:&mut HashMap<u64, u128>, stone: &(u64, u128))
{
    if stone.0 == 0
    {
        *list.entry(1).or_insert(0) += stone.1;
    }
    else if stone.0.ilog10() % 2 == 1
    {
        let split = stone.0.ilog10();
        let pow = 10u64.pow((split + 1) / 2);
        *list.entry(stone.0 / pow).or_insert(0) += stone.1;
        *list.entry(stone.0 % pow).or_insert(0) += stone.1;
    }
    else
    {
        *list.entry(stone.0 * 2024).or_insert(0) += stone.1;
    }
}

fn main()
{
    let mut ln = String::new();
    let _bc = stdin().read_line(&mut ln);
    ln.pop();
    let mut q: HashMap<u64, u128> = HashMap::new();
    let mut r: HashMap<u64, u128> = HashMap::new();
    for i in ln.split(" ").map(|str|str.parse::<u64>().unwrap())
    {
        *q.entry(i).or_insert(0) += 1;
    }
    for _i in 0..75
    {
        for i in q.iter()
        {
            transform(&mut r, &(*i.0, *i.1));
        }
        std::mem::swap(&mut q, &mut r);
        r.clear();
    }
    println!("{}", q.into_values().sum::<u128>());
}
