use std::collections::HashSet;
use std::io::stdin;

fn movehelp(grid :&mut [[u8; 100]; 50], immovable: &mut HashSet<(isize, isize)>, pos:(isize, isize), vec: (isize, isize)) -> bool
{
    if immovable.contains(&pos)
    {
        return false;
    }
    let test = (pos.0 + vec.0, pos.1 + vec.1);
    let (upos, utest) = (((pos.0 as usize), (pos.1 as usize)), ((test.0 as usize), (test.1 as usize)));
    let possible: bool = match grid[utest.1][utest.0]
    {
        35 => {
            immovable.insert(pos);
            false
        },
        46 => {
            true
        },
        3 => {
            (vec.1 == 0 || movehelp(grid, immovable, (test.0 + 1, test.1), vec)) && movehelp(grid, immovable, test, vec)
        },
        4 => {
            (vec.1 == 0 || movehelp(grid, immovable, (test.0 - 1, test.1), vec)) && movehelp(grid, immovable, test, vec)
        },
        _ => {
            movehelp(grid, immovable, test, vec)
        }
    };
    if possible
    {
        let tmp = grid[upos.1][upos.0];
        grid[upos.1][upos.0] = grid[utest.1][utest.0];
        grid[utest.1][utest.0] = tmp;
    }
    return possible;
}

fn moveb(grid :&mut [[u8; 100]; 50], pos:(isize, isize), direc: u8) -> (isize, isize)
{
    let vec = match direc
    {
        // left
        60 => (-1, 0),
        // right
        62 => (1, 0),
        // up
        94 => (0, -1),
        // down
        118 => (0, 1),
        _ignore => pos
    };
    let backup = *grid;
    let mut cache: HashSet<(isize, isize)> = HashSet::new();
    if !movehelp(grid, &mut cache, pos, vec)
    {
        *grid = backup;
        return pos;
    }
    return(pos.0 + vec.0, pos.1 + vec.1);
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut room = [[0u8; 100]; 50];
    let mut start: (isize, isize) = (0, 0);
    let mut tot = 0;
    let mut row: usize = 0;
    let mut ln = String::new();
    let mut bc = stdin().read_line(&mut ln).unwrap();
    while bc > 1
    {
        ln.pop();
        for (i, ch) in ln.chars().enumerate()
        {
            room[row][i * 2] = ch as u8;
            room[row][i * 2 + 1] = room[row][i * 2];
            if room[row][i * 2] == 64
            {
                start = (((i * 2) as isize), (row as isize));
                room[row][i * 2 + 1] = 46;
            }
            else if room[row][i * 2] == 79
            {
                room[row][i * 2] = 3;
                room[row][i * 2 + 1] = 4;
            }
        }
        row += 1;
        ln.clear();
        bc = stdin().read_line(&mut ln).unwrap();
    }
    ln.clear();
    while bc > 0
    {
        ln.pop();
        bc = stdin().read_line(&mut ln).unwrap();
    }
    for ch in ln.chars()
    {
        start = moveb(&mut room, start, ch as u8);
    }
    for (i, u) in room.as_slice().iter().enumerate()
    {
        for (j, _v) in u.as_slice().iter().enumerate().filter(|x|*x.1 == 3u8)
        {
            tot += i * 100 + j;
        }
    }
    println!("{}", tot);
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
