use std::io::stdin;

fn trymake(s: &String, t: &Vec<String>) -> u64
{
    let mut cache: Vec<u64> = Vec::new();
    cache.resize(s.len() + 1, 0);
    cache[0] = 1;
    for i in 0..s.len()
    {
        if cache[i] > 0
        {
            for j in t
            {
                if i + j.len() <= s.len() && s[i..(i + j.len())] == *j
                {
                    cache[i + j.len()] += cache[i];
                }
            }
        }
    }
    return cache[s.len()];
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut tot: u64 = 0;
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
