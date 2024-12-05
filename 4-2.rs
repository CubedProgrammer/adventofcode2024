use std::io::stdin;

fn main()
{
    let target = ['M', 'A', 'S'];
    let rtarget = ['S', 'A', 'M'];
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
    for i in 0..rcnt-2
    {
        for j in (i*ccnt)..((i+1)*ccnt-2)
        {
            if (
                (grid.iter().skip(j).step_by(ccnt+1).take(3).eq(target.iter()) || grid.iter().skip(j).step_by(ccnt+1).take(3).eq(rtarget.iter()))
                &&
                (grid.iter().skip(j+2).step_by(ccnt-1).take(3).eq(target.iter()) || grid.iter().skip(j+2).step_by(ccnt-1).take(3).eq(rtarget.iter()))
            )
            {
                xmas += 1;
            }
        }
    }
    println!("{}", xmas);
}
