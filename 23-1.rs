use std::collections::HashSet;
use std::collections::HashMap;
use std::io::stdin;

fn main_rs(_args: Vec<String>) -> i32
{
    let edges = stdin().lines().map(|x|x.unwrap().split("-").map(|x|String::from(x)).collect::<Vec<String>>()).collect::<HashSet<Vec<String>>>();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut cnt: u64 = 0;
    let (mut dup2, mut dup3): (u64, u64) = (0, 0);
    for e in edges.iter()
    {
        graph.entry(e[0].clone()).or_insert(Vec::new()).push(e[1].clone());
        graph.entry(e[1].clone()).or_insert(Vec::new()).push(e[0].clone());
    }
    for (_n, e) in graph.iter().filter(|x|x.0.starts_with("t"))
    {
        for (i, attached) in e.iter().enumerate()
        {
            for other in e.iter().skip(i + 1)
            {
                let testedge1 = vec![attached.clone(), other.clone()];
                let testedge2 = vec![other.clone(), attached.clone()];
                if edges.contains(&testedge1) || edges.contains(&testedge2)
                {
                    cnt += 1;
                    let q = attached.starts_with("t");
                    let r = other.starts_with("t");
                    if q && r
                    {
                        dup3 += 1;
                    }
                    else if q || r
                    {
                        dup2 += 1;
                    }
                }
            }
        }
    }
    println!("{}", cnt - dup2 / 2 - dup3 / 3);
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
