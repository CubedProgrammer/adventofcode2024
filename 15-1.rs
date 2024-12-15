use std::io::stdin;

fn moveb(grid :&mut [[u8; 50]; 50], pos:(isize, isize), direc: u8) -> (isize, isize)
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
    let mut test = pos;
    while test.0 >= 0 && test.0 < 50 && test.1 >= 0 && test.1 < 50
    {
        if grid[test.1 as usize][test.0 as usize] == 35
        {
            return pos;
        }
        if grid[test.1 as usize][test.0 as usize] == 46
        {
            break;
        }
        test.0 += vec.0;
        test.1 += vec.1;
    }
    while test != pos
    {
        grid[test.1 as usize][test.0 as usize] = grid[(test.1 - vec.1) as usize][(test.0 - vec.0) as usize];
        test.0 -= vec.0;
        test.1 -= vec.1;
    }
    grid[pos.1 as usize][pos.0 as usize] = 46;
    return(pos.0 + vec.0, pos.1 + vec.1);
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut room = [[0u8; 50]; 50];
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
            room[row][i] = ch as u8;
            if room[row][i] == 64
            {
                start = ((i as isize), (row as isize));
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
        for (j, _v) in u.as_slice().iter().enumerate().filter(|x|*x.1 == 79u8)
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
