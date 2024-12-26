use std::collections::HashMap;
use std::io::stdin;

fn buildans(val: &String, cache: &mut HashMap<String, u32>, graph: &HashMap<String, [String; 3]>)
{
    if !cache.contains_key(val)
    {
        buildans(&graph[val][0], cache, graph);
        buildans(&graph[val][2], cache, graph);
        let x = cache[&graph[val][0]];
        let y = cache[&graph[val][2]];
        let op = &graph[val][1];
        let mut z: u32 = 0;
        if *op == "AND"
        {
            z = x & y;
        }
        else if *op == "OR"
        {
            z = x | y;
        }
        else if *op == "XOR"
        {
            z = x ^ y;
        }
        cache.insert(val.clone(), z);
    }
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut start: HashMap<String, u32> = HashMap::new();
    let mut eqgraph: HashMap<String, [String; 3]> = HashMap::new();
    let mut zval: Vec<String> = Vec::new();
    let mut res: u64 = 0;
    let mut ln = String::new();
    let mut bc = stdin().read_line(&mut ln).unwrap();
    while bc > 1
    {
        ln.pop();
        let v: Vec<&str> = ln.split(": ").collect();
        start.insert(String::from(v[0]), v[1].parse().unwrap());
        ln.clear();
        bc = stdin().read_line(&mut ln).unwrap();
    }
    for oln in stdin().lines()
    {
        let  eq = oln.unwrap().split_ascii_whitespace().map(|x|String::from(x)).collect::<Vec<String>>();
        if eq[4].starts_with("z")
        {
            zval.push(eq[4].clone());
        }
        eqgraph.insert(eq[4].clone(), [eq[0].clone(), eq[1].clone(), eq[2].clone()]);
    }
    zval.sort();
    for i in zval.iter().rev()
    {
        buildans(i, &mut start, &eqgraph);
        res <<= 1;
        res |= start[i]as u64;
    }
    println!("{}", res);
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
