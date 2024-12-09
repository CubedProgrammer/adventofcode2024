use std::collections::BTreeSet;
use std::io::stdin;

fn main()
{
    let mut intervals: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
    let mut files: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
    let mut ind_by_size: Vec<BTreeSet<usize>> = Vec::with_capacity(10);
    let mut ind: usize = 0;
    let mut ln = String::new();
    let mut cnt = stdin().read_line(&mut ln).unwrap();
    ln.truncate(cnt - 1);
    ind_by_size.resize(10, BTreeSet::new());
    for (i, ch) in ln.chars().enumerate()
    {
        let len = (ch as usize) - ('0' as usize);
        if i % 2 == 0
        {
            intervals.insert((ind, len, i / 2));
        }
        else
        {
            ind_by_size[len].insert(ind);
        }
        ind += len;
    }
    while intervals.len() > 0
    {
        let curr = intervals.pop_last().unwrap();
        let (small, smalllen) = ind_by_size.iter().enumerate().skip(curr.1).map(|i|(*i.1.first().unwrap_or(&99999999), i.0)).min().unwrap();
        if small < curr.0
        {
            ind_by_size[smalllen].pop_first();
            if smalllen > curr.1
            {
                ind_by_size[smalllen - curr.1].insert(small + curr.1);
            }
            files.insert((small, curr.1, curr.2));
        }
        else
        {
            files.insert(curr);
        }
    }
    cnt = 0;
    for (ind, len, id) in files.iter()
    {
        cnt += id * ((2 * ind + len - 1) * len / 2);
    }
    println!("{}", cnt);
}
