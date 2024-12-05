use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::io::stdin;

fn main()
{
    let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut visited: HashSet<u32> = HashSet::new();
    let mut q: LinkedList<u32> = LinkedList::new();
    let mut tot: u32 = 0;
    let mut ln = String::new();
    let mut bcnt = stdin().read_line(&mut ln).unwrap();
    while bcnt > 1
    {
        ln.pop();
        let rule: Vec<u32> = ln.split("|").map(|s|s.parse().unwrap()).collect();
        for num in rule.iter()
        {
            if !graph.contains_key(num)
            {
                graph.insert(*num, HashSet::new());
            }
        }
        graph.get_mut(&rule[0]).unwrap().insert(rule[1]);
        ln.clear();
        bcnt = stdin().read_line(&mut ln).unwrap();
    }
    for (n, to) in graph.iter_mut()
    {
        visited.clear();
        q.push_back(*n);
        while q.len() > 0
        {
            let curr = q.pop_front().unwrap();
            if visited.contains(&curr)
            {
                continue;
            }
            visited.insert(curr);
            if curr != *n
            {
                to.insert(curr);
            }
        }
    }
    ln.clear();
    bcnt = stdin().read_line(&mut ln).unwrap();
    while bcnt > 0
    {
        ln.pop();
        let nums: Vec<u32> = ln.split(",").map(|s|s.parse().unwrap()).collect();
        if nums.windows(2).all(|s|graph[&s[0]].contains(&s[1]))
        {
            tot += nums[nums.len() / 2];
        }
        ln.clear();
        bcnt = stdin().read_line(&mut ln).unwrap();
    }
    println!("{}", tot);
}