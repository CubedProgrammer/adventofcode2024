use std::collections::HashMap;
use std::io::stdin;

#[derive(Clone, Debug, Hash)]
struct Motion
{
    pub x: i64,
    pub y: i64,
    pub flip: bool
}

impl Motion
{
    fn from(x: i64, y: i64, flip: bool) -> Self
    {
        return Motion{x: x, y: y, flip: flip};
    }
    fn new() -> Self
    {
        return Self::from(0, 0, false);
    }
    fn len(&self) -> u64
    {
        if self.x == 0 && self.y == 0
        {
            1
        }
        else
        {
            (self.x.abs() + self.y.abs()) as u64
        }
    }
}

impl PartialEq for Motion
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.x == other.x && self.y == other.y && self.flip == other.flip;
    }
}

impl Eq for Motion {}

fn moresequence(out: &mut HashMap<Motion, u64>, seq: &Motion, cnt: u64)
{
    *out.entry(Motion::new()).or_insert(0) += seq.len() * cnt;
    if seq.x > 0
    {
        if seq.y > 0
        {
            if seq.flip
            {
                *out.entry(Motion::from(0, 1, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(-1, 0, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(1, -1, false)).or_insert(0) += cnt;
            }
            else
            {
                *out.entry(Motion::from(-1, 1, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(1, 0, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(0, -1, false)).or_insert(0) += cnt;
            }
        }
        else if seq.y < 0
        {
            if seq.flip
            {
                *out.entry(Motion::from(0, 1, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(-1, -1, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(1, 0, false)).or_insert(0) += cnt;
            }
            else
            {
                *out.entry(Motion::from(0, -1, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(1, 1, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(-1, 0, false)).or_insert(0) += cnt;
            }
        }
        else
        {
            *out.entry(Motion::from(0, 1, false)).or_insert(0) += cnt;
            *out.entry(Motion::from(0, -1, false)).or_insert(0) += cnt;
        }
    }
    else if seq.x < 0
    {
        if seq.y > 0
        {
            if seq.flip
            {
                *out.entry(Motion::from(-1, 1, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(-1, 0, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(2, -1, true)).or_insert(0) += cnt;
            }
            else
            {
                *out.entry(Motion::from(-2, 1, true)).or_insert(0) += cnt;
                *out.entry(Motion::from(1, 0, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(1, -1, false)).or_insert(0) += cnt;
            }
        }
        else if seq.y < 0
        {
            if seq.flip
            {
                *out.entry(Motion::from(-1, 0, false)).or_insert(0) += cnt;
                *out.entry(Motion::from(-1, 1, true)).or_insert(0) += cnt;
                *out.entry(Motion::from(2, -1, true)).or_insert(0) += cnt;
            }
            else
            {
                *out.entry(Motion::from(-2, 1, true)).or_insert(0) += cnt;
                *out.entry(Motion::from(1, -1, true)).or_insert(0) += cnt;
                *out.entry(Motion::from(1, 0, false)).or_insert(0) += cnt;
            }
        }
        else
        {
            *out.entry(Motion::from(-2, 1, true)).or_insert(0) += cnt;
            *out.entry(Motion::from(2, -1, true)).or_insert(0) += cnt;
        }
    }
    else
    {
        if seq.y > 0
        {
            *out.entry(Motion::from(-1, 1, false)).or_insert(0) += cnt;
            *out.entry(Motion::from(1, -1, false)).or_insert(0) += cnt;
        }
        else if seq.y < 0
        {
            *out.entry(Motion::from(-1, 0, false)).or_insert(0) += cnt;
            *out.entry(Motion::from(1, 0, false)).or_insert(0) += cnt;
        }
    }
}

fn sequence(out: &mut HashMap<Motion, u64>, x: u64, y: u64)
{
    let mut step: Motion = Motion::new();
    *out.entry(step.clone()).or_insert(0) += 1;
    if x == y
    {
        return;
    }
    let pos: [(i64, i64); 11] = [(1, 3), (0, 2), (1, 2), (2, 2), (0, 1), (1, 1), (2, 1), (0, 0), (1, 0), (2, 0), (2, 3)];
    let (xp, yp) = (pos[x as usize], pos[y as usize]);
    step.x = yp.0 - xp.0;
    step.y = yp.1 - xp.1;
    step.flip = ((x == 1 || x == 4 || x == 7) && (y == 0 || y == 10)) || ((y == 1 || y == 4 || y == 7) && (x == 0 || x == 10));
    *out.entry(step).or_insert(0) += 1;
}

fn shortest(mut num: u64) -> u64
{
    let mut seq: HashMap<Motion, u64> = HashMap::new();
    let mut nseq: HashMap<Motion, u64> = HashMap::new();
    let mut lastdig: u64 = 10;
    for _i in 0..3
    {
        let dig = num % 10;
        sequence(&mut seq, dig, lastdig);
        lastdig = dig;
        num /= 10;
    }
    sequence(&mut seq, 10, lastdig);
    for _i in 0..25
    {
        for i in seq.iter()
        {
            moresequence(&mut nseq, i.0, *i.1);
        }
        std::mem::swap(&mut seq, &mut nseq);
        nseq.clear();
    }
    return seq.iter().map(|x|x.0.len() * x.1).sum::<u64>();
}

fn main_rs(_args: Vec<String>) -> i32
{
    let codearr: Vec<u64> = stdin().lines().map(|x|{let y=x.unwrap();y[0..y.len()-1].parse().unwrap()}).collect();
    println!("{}", codearr.iter().map(|x|*x * shortest(*x)).sum::<u64>());
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
