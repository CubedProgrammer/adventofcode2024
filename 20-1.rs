use std::io::stdin;

static BLOCKED: char = '#';
static END: char = 'E';

fn find_ind<T: std::cmp::PartialEq>(grid: &Vec<Vec<T>>, v: &T) -> Option<(usize, usize)>
{
    for (i, q) in grid.iter().enumerate()
    {
        for (j, r) in q.iter().enumerate()
        {
            if *r == *v
            {
                return Some((j, i));
            }
        }
    }
    return None;
}

fn builddist(g: &Vec<Vec<char>>, d: &mut Vec<Vec<u64>>, start: (usize, usize), currdist: u64)
{
    if start.1 < g.len() && start.0 < g[start.0].len() && g[start.1][start.0] != BLOCKED && d[start.1][start.0] > currdist
    {
        d[start.1][start.0] = currdist;
        builddist(g, d, (start.0 + 1, start.1), currdist + 1);
        builddist(g, d, (start.0, start.1 + 1), currdist + 1);
        builddist(g, d, (start.0 - 1, start.1), currdist + 1);
        builddist(g, d, (start.0, start.1 - 1), currdist + 1);
    }
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut saves = [0u64; 20000];
    let grid = stdin().lines().map(|x|x.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut dist: Vec<Vec<u64>> = Vec::new();
    let endpos = find_ind(&grid, &END).unwrap();
    let vec: [[isize; 2]; 8] = [[2, 0], [1, 1], [0, 2], [-1, 1], [-2, 0], [-1, -1], [0, -2], [1, -1]];
    dist.resize(grid.len(), Vec::new());
    for (q, r) in grid.iter().zip(dist.iter_mut())
    {
        r.resize(q.len(), 99999999);
    }
    builddist(&grid, &mut dist, endpos, 0);
    for (i, r) in dist.iter().enumerate()
    {
        for (j, v) in r.iter().enumerate().filter(|x|*x.1 < 30000)
        {
            for movement in vec.iter().map(|x|((j as isize) + x[0], (i as isize) + x[1]))
                .filter(|x|x.1 >= 0 && x.1 < grid.len()as isize && x.0 >= 0 && x.0 < r.len()as isize)
            {
                let quick = dist[movement.1 as usize][movement.0 as usize];
                if quick + 2 < *v
                {
                    saves[(*v - quick - 2)as usize] += 1;
                }
            }
        }
    }
    println!("{}", saves.as_slice().iter().skip(100).sum::<u64>());
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
