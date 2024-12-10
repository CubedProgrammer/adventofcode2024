use std::collections::HashSet;
use std::collections::LinkedList;
use std::io::stdin;

fn cnt(grid: &Vec<Vec<char>>, col: usize, start: (usize, usize)) -> usize
{
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut q: LinkedList<(usize, usize, char)> = LinkedList::new();
    let mut tot: usize = 0;
    q.push_back((start.0, start.1, '0'));
    while q.len() > 0
    {
        let pos = q.pop_front().unwrap();
        let next = char::from_u32((pos.2 as u32) + 1).unwrap();
        if visited.contains(&(pos.0, pos.1)) || pos.0 >= col || pos.1 >= grid.len() || pos.2 != grid[pos.1][pos.0]
        {continue;}
        visited.insert((pos.0, pos.1));
        if pos.2 == '9'
        {
            tot += 1;
            continue;
        }
        q.push_back((pos.0 + 1, pos.1, next));
        q.push_back((pos.0 - 1, pos.1, next));
        q.push_back((pos.0, pos.1 + 1, next));
        q.push_back((pos.0, pos.1 - 1, next));
    }
    return tot;
}

fn main()
{
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut zeros: Vec<(usize, usize)> = Vec::new();
    let mut rcnt: usize = 0;
    let mut ccnt: usize = 0;
    let mut ln = String::new();
    let mut bcnt = stdin().read_line(&mut ln).unwrap();
    while bcnt > 0
    {
        ln.pop();
        ccnt = ln.len();
        for (i, _ch) in ln.chars().enumerate().filter(|x|x.1 == '0')
        {
            zeros.push((i, rcnt));
        }
        grid.push(ln.chars().collect());
        rcnt += 1;
        ln.clear();
        bcnt = stdin().read_line(&mut ln).unwrap();
    }
    let ans: usize = zeros.iter().map(|p|cnt(&grid, ccnt, *p)).sum();
    println!("{}", ans);
}
