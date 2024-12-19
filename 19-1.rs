use std::io::stdin;

fn trymake(s: &String, t: &Vec<String>) -> u32
{
    let mut cache: Vec<bool> = Vec::new();
    cache.resize(s.len() + 1, false);
    cache[0] = true;
    for i in 0..s.len()
    {
        if cache[i]
        {
            for j in t
            {
                if i + j.len() <= s.len() && s[i..(i + j.len())] == *j
                {
                    cache[i + j.len()] = true;
                }
            }
        }
    }
    if cache[s.len()]
    {
        return 1;
    }
    else
    {
        return 0;
    }
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut tot: u32 = 0;
    let mut ln = String::new();
    let mut _bc = stdin().read_line(&mut ln).unwrap();
    let avail: Vec<String> = ln.split(", ").map(|x|String::from(x)).collect();
    _bc = stdin().read_line(&mut ln).unwrap();
    ln.clear();
    _bc = stdin().read_line(&mut ln).unwrap();
    while _bc > 0
    {
        ln.pop();
        tot += trymake(&ln, &avail);
        ln.clear();
        _bc = stdin().read_line(&mut ln).unwrap();
    }
    println!("{}", tot);
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
