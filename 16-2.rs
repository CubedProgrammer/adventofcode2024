use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;

type Edgemap = HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>;

fn count(visited:&mut HashSet<(usize, usize)>, map: &Edgemap, dest: (usize, usize, usize))
{
    match map.get(&dest)
    {
        Some(v) => v.iter().for_each(|x|count(visited, map, *x)),
        None => {}
    }
    visited.insert((dest.0, dest.1));
}

fn shortest(grid: &Vec<Vec<u8>>, d: &mut Vec<Vec<[u32; 4]>>, s: (usize, usize, usize)) -> Edgemap
{
    let big: u32 = 999999999;
    let costs: [[u32; 4]; 4] = [[1, 1001, 2001, 1001], [1001, 1, 1001, 2001], [2001, 1001, 1, 1001], [1001, 2001, 1001, 1]];
    let mut q:BinaryHeap<(u32, (usize, usize, usize), (usize, usize, usize))> = BinaryHeap::new();
    let mut back: Edgemap = HashMap::new();
    q.push((big, s, s));
    while q.len() > 0
    {
        let stuff = q.pop().unwrap();
        let curr = stuff.1;
        if curr.1 >= grid.len() || curr.0 >= grid[curr.1].len() || grid[curr.1][curr.0] == 35 || big - stuff.0 > d[curr.1][curr.0][curr.2]
        {
            continue;
        }
        if stuff.2 != curr
        {
            back.entry(curr).or_insert(Vec::new()).push(stuff.2);
        }
        if big - stuff.0 == d[curr.1][curr.0][curr.2]
        {
            continue;
        }
        d[curr.1][curr.0][curr.2] = big - stuff.0;
        let currcost = d[curr.1][curr.0][curr.2];
        q.push((big - currcost - costs[curr.2][0], (curr.0 + 1, curr.1, 0), curr));
        q.push((big - currcost - costs[curr.2][1], (curr.0, curr.1 - 1, 1), curr));
        q.push((big - currcost - costs[curr.2][2], (curr.0 - 1, curr.1, 2), curr));
        q.push((big - currcost - costs[curr.2][3], (curr.0, curr.1 + 1, 3), curr));
    }
    return back;
}

fn main_rs(_args: Vec<String>) -> i32
{
    let startch = 'S';
    let endch = 'E';
    let mut maze: Vec<Vec<u8>> = Vec::new();
    let mut dist: Vec<Vec<[u32; 4]>> = Vec::new();
    let (mut src, mut dest) = ((0, 0, 0), (0, 0));
    let mut ln = String::new();
    let mut bc = stdin().read_line(&mut ln).unwrap();
    while bc > 0
    {
        ln.pop();
        match ln.find(startch)
        {
            Some(ind) => src = (ind, maze.len(), 0),
            None => {}
        }
        match ln.find(endch)
        {
            Some(ind) => dest = (ind, maze.len()),
            None => {}
        }
        maze.push(ln.chars().map(|x|(x as u8)).collect());
        let mut tmp: Vec<[u32; 4]> = Vec::new();
        tmp.resize(maze.last().unwrap().len(), [99999999; 4]);
        dist.push(tmp);
        ln.clear();
        bc = stdin().read_line(&mut ln).unwrap();
    }
    let backtrack = shortest(&maze, &mut dist, src);
    let (mut ind, mut small) = (0usize, dist[dest.1][dest.0][0]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for i in 1..4
    {
        if dist[dest.1][dest.0][i] < small
        {
            ind = i;
            small = dist[dest.1][dest.0][i];
        }
    }
    count(&mut visited, &backtrack, (dest.0, dest.1, ind));
    println!("{}", visited.len());
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
