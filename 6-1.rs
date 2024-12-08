use std::io::BufReader;
use std::io::Read;
use std::io::stdin;

fn main()
{
    let mut reader = BufReader::new(stdin());
    let mut grid:Vec<u8> = Vec::new();
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
    while ccnt as isize > posx && posx >= 0 && rcnt as isize > posy && posy >= 0
    {
        let ind = (posy as usize) * ccnt + (posx as usize);
        if grid[ind] == 35
        {
            posx -= dir.0;
            posy -= dir.1;
            let nx = dir.1 * -1;
            let ny = dir.0;
            dir.0 = nx;
            dir.1 = ny;
        }
        else
        {
            grid[ind] = 69u8;
        }
        posx += dir.0;
        posy += dir.1;
    }
    println!("{}", grid.iter().filter(|e|**e == 69u8).count());
}
