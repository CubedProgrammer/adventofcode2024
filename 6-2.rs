use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufReader;
use std::io::Read;
use std::io::stdin;

fn turn(v: &(isize, isize)) -> (isize, isize)
{
    return(v.1 * -1, v.0);
}

fn main()
{
    let mut reader = BufReader::new(stdin());
    let mut grid:Vec<u8> = Vec::new();
    let mut dirmap: HashMap<(isize, isize), u8> = HashMap::new();
    let mut obstacle: HashSet<(isize, isize)>= HashSet::new();
    let mut tot = reader.read_to_end(&mut grid).unwrap();
    grid.retain(|e|*e != 10u8);
    let rcnt = tot - grid.len();
    tot = grid.len();
    let ccnt = tot / rcnt;
    let mut dir = (0, 0);
    let mut start: usize = 0;
    for (i, ch) in grid.iter().enumerate()
    {
        if *ch != 46 && *ch != 35
        {
            match *ch
            {
                94 => dir.1 = -1,
                60 => dir.0 = -1,
                62 => dir.0 = 1,
                118 => dir.1 = 1,
                _ => {}
            }
            start = i;
            break;
        }
    }
    let (mut posx, mut posy) = ((start % ccnt) as isize, (start / ccnt) as isize);
    let ogpos = (posx, posy);
    let ogdir = dir;
    dirmap.insert((0, -1), 1u8);
    dirmap.insert((1, 0), 2u8);
    dirmap.insert((0, 1), 3u8);
    dirmap.insert((-1, 0), 4u8);
    for obsind in 0..grid.len()
    {
        if grid[obsind] != 35
        {
            grid[obsind] = 35;
        }
        else{continue;}
        while ccnt as isize > posx && posx >= 0 && rcnt as isize > posy && posy >= 0
        {
            let ind = (posy as usize) * ccnt + (posx as usize);
            if grid[ind] == 35
            {
                posx -= dir.0;
                posy -= dir.1;
                dir = turn(&dir);
            }
            else
            {
                let dirnum = dirmap[&dir];
                if grid[ind] > 32
                {
                    grid[ind] = 0;
                }
                if (grid[ind] >> dirnum & 1) == 1
                {
                    obstacle.insert(((obsind % ccnt) as isize, (obsind / ccnt) as isize));
                    break;
                }
                grid[ind] |= 1 << dirnum;
                posx += dir.0;
                posy += dir.1;
            }
        }
        grid[obsind] = 46;
        for sq in grid.iter_mut()
        {
            if *sq != 35
            {
                *sq = 46;
            }
        }
        posx = ogpos.0;
        posy = ogpos.1;
        dir = ogdir;
    }
    obstacle.remove(&ogpos);
    println!("{}", obstacle.len());
}
