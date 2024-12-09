use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;

fn vectorize(v1: &(usize, usize), v2: &(usize, usize)) -> (isize, isize)
{
    return((v2.0 as isize) - (v1.0 as isize), (v2.1 as isize) - (v1.1 as isize));
}

fn scale(set: &mut HashSet<(usize, usize)>, r: usize, c: usize, from: &(usize, usize), vec: &(isize, isize))
{
    let mut pos: (isize, isize) = ((from.0 as isize), (from.1 as isize));
    while pos.0 >= 0 && pos.1 >= 0 && pos.0 < (c as isize) && pos.1 < (r as isize)
    {
        set.insert(((pos.0 as usize), (pos.1 as usize)));
        pos.0 += vec.0;
        pos.1 += vec.1;
    }
}

fn main()
{
    let (mut rcnt, mut ccnt) : (usize, usize) = (0, 0);
    let mut antenna: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinode: HashSet<(usize, usize)> = HashSet::new();
    let mut currln = String::new();
    let mut bcnt = stdin().read_line(&mut currln).unwrap();
    while bcnt > 0
    {
        rcnt += 1;
        currln.pop();
        ccnt = currln.len();
        for (i, ch) in currln.chars().enumerate()
        {
            if ch != '.'
            {
                antenna.entry(ch).or_insert(Vec::new()).push((i, rcnt - 1));
            }
        }
        currln.clear();
        bcnt = stdin().read_line(&mut currln).unwrap();
    }
    for arr in antenna.values()
    {
        for (i, pos) in arr.iter().enumerate()
        {
            for other in arr.iter().skip(i + 1)
            {
                scale(&mut antinode, rcnt, ccnt, &pos, &vectorize(other, pos));
                scale(&mut antinode, rcnt, ccnt, &other, &vectorize(pos, other));
            }
        }
    }
    println!("{}", antinode.len());
}
