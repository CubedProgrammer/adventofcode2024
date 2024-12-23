use std::collections::HashSet;
use std::collections::HashMap;
use std::io::stdin;

fn main_rs(_args: Vec<String>) -> i32
{
    let edges = stdin().lines().map(|x|x.unwrap().split("-").map(|x|String::from(x)).collect::<Vec<String>>()).collect::<HashSet<Vec<String>>>();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut big: Vec<String> = Vec::new();
    for e in edges.iter()
    {
        graph.entry(e[0].clone()).or_insert(Vec::new()).push(e[1].clone());
        graph.entry(e[1].clone()).or_insert(Vec::new()).push(e[0].clone());
    }
    for (n, e) in graph.iter().filter(|x|x.0.starts_with("t"))
    {
        if big.len() > e.len()
        {
            continue;
        }
        for (i, attached) in e.iter().enumerate()
        {
            if big.len() > e.len() - i
            {
                continue;
            }
            let mut part: Vec<String> = Vec::new();
            part.push(n.clone());
            part.push(attached.clone());
            for other in e.iter().skip(i + 1)
            {
                let mut connected: bool = true;
                for tmp in part.iter().skip(1)
                {
                    let testedge1 = vec![tmp.clone(), other.clone()];
                    let testedge2 = vec![other.clone(), tmp.clone()];
                    if !edges.contains(&testedge1) && !edges.contains(&testedge2)
                    {
                        connected = false;
                        break;
                    }
                }
                if connected
                {
                    part.push(other.clone());
                }
            }
            if part.len() > big.len()
            {
                big = part;
            }
        }
    }
    big.sort();
    let mut notfirst: bool = false;
    for i in big
    {
        if notfirst
        {
            print!(",{}", i);
        }
        else
        {
            print!("{}", i);
            notfirst = true;
        }
    }
    println!("");
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
