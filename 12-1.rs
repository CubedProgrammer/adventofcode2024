use std::collections::HashSet;
use std::collections::LinkedList;
use std::io::stdin;

fn region(g: &mut Vec<Vec<char>>, r: usize, c: usize) -> usize
{
    let mut edges: HashSet<(usize, usize, bool)> = HashSet::new();
    let mut q: LinkedList<(usize, usize)> = LinkedList::new();
    let mut cnt: usize = 0;
    let test = g[r][c];
    q.push_back((c, r));
    while q.len() > 0
    {
        let curr = q.pop_front().unwrap();
        if curr.0 >= g[0].len() || curr.1 >= g.len()
        {continue;}
        if g[curr.1][curr.0] != test
        {continue;}
        g[curr.1][curr.0] = ' ';
        for i in [(curr.0, curr.1, false), (curr.0, curr.1, true), (curr.0 + 1, curr.1, false), (curr.0, curr.1 + 1, true)]
        {
            if !edges.remove(&i)
            {
                edges.insert(i);
            }
        }
        q.push_back((curr.0, curr.1 + 1));
        q.push_back((curr.0 + 1, curr.1));
        q.push_back((curr.0, curr.1 - 1));
        q.push_back((curr.0 - 1, curr.1));
        cnt += 1;
    }
    cnt * edges.len()
}

fn main()
{
    let mut grid:Vec<Vec<char>> = Vec::new();
    let mut tot: usize = 0;
    let mut ln = String::new();
    let mut br = stdin().read_line(&mut ln).unwrap();
    while br > 0
    {
        ln.pop();
        grid.push(ln.chars().collect());
        ln.clear();
        br = stdin().read_line(&mut ln).unwrap();
    }
    for i in 0..grid.len()
    {
        for j in 0..grid[i].len()
        {
            if grid[i][j] != ' '
            {
                tot += region(&mut grid, i, j);
            }
        }
    }
    println!("{}", tot);
}
