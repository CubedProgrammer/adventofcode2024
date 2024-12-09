use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;

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
                let (w, x, y, z) = (2 * pos.0, 2 * pos.1, 2 * other.0, 2 * other.1);
                if w >= other.0 && x >= other.1 && w - other.0 < ccnt && x - other.1 < rcnt
                {
                    antinode.insert((w - other.0, x - other.1));
                }
                if y >= pos.0 && z >= pos.1 && y - pos.0 < ccnt && z - pos.1 < rcnt
                {
                    antinode.insert((y - pos.0, z - pos.1));
                }
            }
        }
    }
    println!("{}", antinode.len());
}
