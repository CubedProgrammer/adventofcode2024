use std::io::stdin;

fn distance(x: u32, y: u32) -> u32
{
    if x == y
    {
        return 1;
    }
    let pos: [(u32, u32); 11] = [(1, 3), (0, 2), (1, 2), (2, 2), (0, 1), (1, 1), (2, 1), (0, 0), (1, 0), (2, 0), (2, 3)];
    let (xp, yp) = (pos[x as usize], pos[y as usize]);
    let mut dist: u32 = xp.0.abs_diff(yp.0) + xp.1.abs_diff(yp.1);
    if (x == 1 || x == 4 || x == 7) && (y == 0 || y == 10)
    {
        dist += 2;
    }
    else if (y == 1 || y == 4 || y == 7) && (x == 0 || x == 10)
    {
        dist += 4;
    }
    if xp.0 > yp.0
    {
        if xp.1 > yp.1
        {
            dist += 19;
        }
        else if xp.1 < yp.1
        {
            dist += 19;
        }
        else
        {
            dist += 17;
        }
    }
    else if xp.0 < yp.0
    {
        if xp.1 > yp.1
        {
            dist += 17;
        }
        else if xp.1 < yp.1
        {
            dist += 15;
        }
        else
        {
            dist += 9;
        }
    }
    else
    {
        if xp.1 > yp.1
        {
            dist += 11;
        }
        else
        {
            dist += 15;
        }
    }
    return dist;
}

fn shortest(mut num: u32) -> u32
{
    let mut tot: u32 = 0;
    let mut lastdig: u32 = 10;
    for _i in 0..3
    {
        let dig = num % 10;
        tot += distance(dig, lastdig);
        lastdig = dig;
        num /= 10;
    }
    return tot + distance(10, lastdig);
}

fn main_rs(_args: Vec<String>) -> i32
{
    let codearr: Vec<u32> = stdin().lines().map(|x|{let y=x.unwrap();y[0..y.len()-1].parse().unwrap()}).collect();
    println!("{}", codearr.iter().map(|x|*x * shortest(*x)).sum::<u32>());
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
