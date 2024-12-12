use std::collections::LinkedList;
use std::io::stdin;

fn region(g: &mut Vec<Vec<char>>, r: usize, c: usize) -> usize
{
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
        g[curr.1][curr.0] = '+';
        q.push_back((curr.0, curr.1 + 1));
        q.push_back((curr.0 + 1, curr.1));
        q.push_back((curr.0, curr.1 - 1));
        q.push_back((curr.0 - 1, curr.1));
        cnt += 1;
    }
    let (mut u, mut v) = (true, true);
    let mut edges: usize = 0;
    for i in 0..g.len()
    {
        u = true;
        v = true;
        for j in 0..g[i].len()
        {
            if g[i][j] == '+'
            {
                if i == 0 || g[i-1][j] != '+'
                {
                    if u
                    {
                        edges += 1;
                    }
                    u = false;
                }
                else
                {
                    u = true;
                }
                if i == g.len() - 1 || g[i+1][j] != '+'
                {
                    if v
                    {
                        edges += 1;
                    }
                    v = false;
                }
                else
                {
                    v = true;
                }
            }
            else
            {
                u = true;
                v = true;
            }
        }
    }
    for j in 0..g[0].len()
    {
        u = true;
        v = true;
        for i in 0..g.len()
        {
            if g[i][j] == '+'
            {
                if j == 0 || g[i][j-1] != '+'
                {
                    if u
                    {
                        edges += 1;
                    }
                    u = false;
                }
                else
                {
                    u = true;
                }
                if j == g[0].len() - 1 || g[i][j+1] != '+'
                {
                    if v
                    {
                        edges += 1;
                    }
                    v = false;
                }
                else
                {
                    v = true;
                }
            }
            else
            {
                u = true;
                v = true;
            }
        }
    }
    for i in g.iter_mut()
    {
        for j in i.iter_mut().filter(|x|**x == '+')
        {
            *j = ' ';
        }
    }
    cnt * edges
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
