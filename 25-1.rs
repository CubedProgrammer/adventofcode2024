use std::io::stdin;

fn main_rs(_args: Vec<String>) -> i32
{
    let mut locks: Vec<[isize; 5]> = Vec::new();
    let mut keys: Vec<[isize; 5]> = Vec::new();
    let mut tot: u32 = 0;
    let invec = stdin().lines().map(|x|x.unwrap()).collect::<Vec<String>>();
    for i in invec.chunks(8)
    {
        let iskey: bool = i[0] == ".....".to_string();
        let mut arr: [isize; 5] = [0; 5];
        for j in i.iter().take(7)
        {
            for (k, ch) in j.chars().enumerate()
            {
                if ch != '.'
                {
                    arr[k] += 1;
                }
            }
        }
        if iskey
        {
            keys.push(arr);
        }
        else
        {
            locks.push(arr);
        }
    }
    for i in locks.iter()
    {
        for j in keys.iter()
        {
            if i.iter().zip(j.iter()).all(|x|7 >= *x.0 + *x.1)
            {
                tot += 1;
            }
        }
    }
    println!("{}", tot);
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
