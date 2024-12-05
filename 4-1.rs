use std::io::stdin;

fn main()
{
    let target = ['X', 'M', 'A', 'S'];
    let rtarget = ['S', 'A', 'M', 'X'];
    let mut ln = String::new();
    let mut bcnt = stdin().read_line(&mut ln).unwrap();
    let ccnt = bcnt - 1;
    let mut xmas: usize = 0;
    while bcnt > 0
    {
        ln.pop();
        bcnt = stdin().read_line(&mut ln).unwrap();
    }
    let grid: Vec<char> = ln.chars().collect();
    let rcnt = grid.len() / ccnt;
    for r in grid.chunks(ccnt)
    {
        xmas += r.windows(4).filter(|v| **v == rtarget || **v == target).count();
    }
    for i in 0..grid.len()-3*rcnt
    {
        if grid.iter().skip(i).step_by(ccnt).take(4).eq(target.iter())
        || grid.iter().skip(i).step_by(ccnt).take(4).eq(rtarget.iter())
        {
            xmas += 1;
        }
    }
    for i in 0..rcnt-3
    {
        for j in (i*ccnt)..((i+1)*ccnt-3)
        {
            if grid.iter().skip(j).step_by(ccnt+1).take(4).eq(target.iter())
            || grid.iter().skip(j).step_by(ccnt+1).take(4).eq(rtarget.iter())
            {
                xmas += 1;
            }
            if grid.iter().skip(j+3).step_by(ccnt-1).take(4).eq(target.iter())
            || grid.iter().skip(j+3).step_by(ccnt-1).take(4).eq(rtarget.iter())
            {
                xmas += 1;
            }
        }
    }
    println!("{}", xmas);
}
