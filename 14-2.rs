use std::collections::HashSet;
use std::io::stdin;

fn parse(s: &String) -> ((i32, i32), (i32, i32))
{
    let nums = s.split(" ").map(|x|x[2..].split(",").map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    return((nums[0][0], nums[0][1]), (nums[1][0], nums[1][1]));
}

fn nooverlap(b: &Vec<((i32, i32), (i32, i32))>) -> bool
{
    let unique = b.iter().map(|x|x.0).collect::<HashSet<(i32, i32)>>();
    return b.len() == unique.len();
}

fn increment(p:&mut (i32, i32), v:&(i32, i32))
{
    p.0 = ((p.0 + v.0) % 101 + 101) % 101;
    p.1 = ((p.1 + v.1) % 103 + 103) % 103;
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut bots:Vec<((i32, i32), (i32, i32))> = Vec::new();
    let mut ln = String::new();
    let mut bc = stdin().read_line(&mut ln).unwrap();
    while bc > 0
    {
        ln.pop();
        bots.push(parse(&ln));
        ln.clear();
        bc = stdin().read_line(&mut ln).unwrap();
    }
    for i in 0..10000
    {
        for j in bots.iter_mut()
        {
            increment(&mut j.0, &j.1);
        }
        if nooverlap(&bots)
        {
            println!("{}", i + 1);
            break;
        }
    }
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
