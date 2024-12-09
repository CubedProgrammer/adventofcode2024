use std::io::stdin;

fn main()
{
    let mut disk: Vec<i64> = Vec::new();
    let mut ln = String::new();
    let mut cnt = stdin().read_line(&mut ln).unwrap();
    ln.truncate(cnt - 1);
    for (i, ch) in ln.chars().enumerate()
    {
        for _j in 0..((ch as i64) - ('0' as i64))
        {
            disk.push((((i as i64) + 1) % 2) * ((i as i64) / 2 + 1));
        }
    }
    cnt = disk.len();
    for i in 0..cnt
    {
        if disk[i] == 0
        {
            cnt -= 1;
            while disk[cnt] == 0
            {
                cnt -= 1;
            }
            if i > cnt
            {
                break;
            }
            else
            {
                disk.swap(i, cnt);
            }
        }
    }
    disk.truncate(cnt + 1);
    println!("{}", disk.iter().enumerate().map(|x|(*x.1 - 1) * (x.0 as i64)).sum::<i64>());
}
