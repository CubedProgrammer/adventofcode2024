use std::collections::LinkedList;
use std::io::stdin;

fn main_rs(_args: Vec<String>) -> i32
{
    let mut grid = [[false; 71]; 71];
    let mut ln = String::new();
    for _i in 0..1024
    {
        let _discard = stdin().read_line(&mut ln);
        ln.pop();
        let nums: Vec<usize> = ln.split(",").map(|x|x.parse().unwrap()).collect();
        grid[nums[1]][nums[0]] = true;
        ln.clear();
    }
    let mut q: LinkedList<(usize, usize, usize)> = LinkedList::new();
    q.push_back((0, 0, 0));
    while !q.is_empty()
    {
        let pos = q.pop_front().unwrap();
        if pos.1 >= 71 || pos.0 >= 71 || grid[pos.1][pos.0]
        {
            continue;
        }
        if pos.1 == 70 && pos.0 == 70
        {
            println!("{}", pos.2);
            continue;
        }
        grid[pos.1][pos.0] = true;
        q.push_back((pos.0 + 1, pos.1, pos.2 + 1));
        q.push_back((pos.0, pos.1 + 1, pos.2 + 1));
        q.push_back((pos.0 - 1, pos.1, pos.2 + 1));
        q.push_back((pos.0, pos.1 - 1, pos.2 + 1));
    }
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
