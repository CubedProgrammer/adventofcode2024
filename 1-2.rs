use std::io;
use std::collections::HashMap;

fn main()
{
    let mut ln: String = "".to_string();
    let mut first: HashMap<u32, u32> = HashMap::new();
    let mut second: HashMap<u32, u32> = HashMap::new();
    let mut tot: u32 = 0;
    let mut bc = io::stdin().read_line(&mut ln).unwrap();
    while bc > 0
    {
        ln.pop();
        let mut parts = ln.split("   ");
        let x: u32 = parts.next().unwrap().parse().unwrap();
        let y: u32 = parts.next().unwrap().parse().unwrap();
        if first.contains_key(&x)
        {
            *first.get_mut(&x).unwrap() += 1;
        }
        else
        {
            first.insert(x, 1);
        }
        if second.contains_key(&y)
        {
            *second.get_mut(&y).unwrap() += 1;
        }
        else
        {
            second.insert(y, 1);
        }
        ln.clear();
        bc = io::stdin().read_line(&mut ln).unwrap();
    }
    for (u, v) in first.iter()
    {
        tot += second.get(u).unwrap_or(&0) * u * v;
    }
    println!("{}", tot);
}
