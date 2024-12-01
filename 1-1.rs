use std::io;

fn main()
{
    let mut ln: String = "".to_string();
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();
    let mut tot: u32 = 0;
    let mut bc = io::stdin().read_line(&mut ln).unwrap();
    while bc > 0
    {
        ln.pop();
        let mut parts = ln.split("   ");
        first.push(parts.next().unwrap().parse().unwrap());
        second.push(parts.next().unwrap().parse().unwrap());
        ln.clear();
        bc = io::stdin().read_line(&mut ln).unwrap();
    }
    first.sort();
    second.sort();
    for (u, v) in first.iter().zip(second.iter())
    {
        if u > v
        {
            tot += u - v;
        }
        else
        {
            tot += v - u;
        }
    }
    println!("{}", tot);
}
