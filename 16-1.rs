use std::collections::LinkedList;
use std::io::stdin;

fn main_rs(_args: Vec<String>) -> i32
{
    let startch = 'S';
    let endch = 'E';
    let costs: [[u32; 4]; 4] = [[1, 1001, 2001, 1001], [1001, 1, 1001, 2001], [2001, 1001, 1, 1001], [1001, 2001, 1001, 1]];
    let mut maze: Vec<Vec<u8>> = Vec::new();
    let mut dist: Vec<Vec<u32>> = Vec::new();
    let (mut src, mut dest) = ((0, 0), (0, 0));
    let mut ln = String::new();
    let mut bc = stdin().read_line(&mut ln).unwrap();
    while bc > 0
    {
        ln.pop();
        match ln.find(startch)
        {
            Some(ind) => src = (ind, maze.len()),
            None => {}
        }
        match ln.find(endch)
        {
            Some(ind) => dest = (ind, maze.len()),
            None => {}
        }
        maze.push(ln.chars().map(|x|(x as u8)).collect());
        let mut tmp: Vec<u32> = Vec::new();
        tmp.resize(maze.last().unwrap().len(), 99999999);
        dist.push(tmp);
        ln.clear();
        bc = stdin().read_line(&mut ln).unwrap();
    }
    let mut q:LinkedList<(usize, usize, u32, usize)> = LinkedList::new();
    q.push_back((src.0, src.1, 0, 0));
    while q.len() > 0
    {
        let curr = q.pop_front().unwrap();
        if curr.1 >= maze.len() || curr.0 >= maze[curr.1].len() || maze[curr.1][curr.0] == 35 || curr.2 >= dist[curr.1][curr.0]
        {
            continue;
        }
        dist[curr.1][curr.0] = curr.2;
        let currcost = dist[curr.1][curr.0];
        q.push_back((curr.0 + 1, curr.1, currcost + costs[curr.3][0], 0));
        q.push_back((curr.0, curr.1 - 1, currcost + costs[curr.3][1], 1));
        q.push_back((curr.0 - 1, curr.1, currcost + costs[curr.3][2], 2));
        q.push_back((curr.0, curr.1 + 1, currcost + costs[curr.3][3], 3));
    }
    println!("{}", dist[dest.1][dest.0]);
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
