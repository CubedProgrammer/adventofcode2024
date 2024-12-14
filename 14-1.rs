use std::io::stdin;

fn parse(s: &String) -> ((i32, i32), (i32, i32))
{
    let nums = s.split(" ").map(|x|x[2..].split(",").map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    return((nums[0][0], nums[0][1]), (nums[1][0], nums[1][1]));
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut ln = String::new();
    let mut bc = stdin().read_line(&mut ln).unwrap();
    let mut qcnt = [0; 4];
    while bc > 0
    {
        ln.pop();
        let (pos, vec) = parse(&ln);
        let noloop = (pos.0 + vec.0 * 100, pos.1 + vec.1 * 100);
        let fin = ((noloop.0 % 101 + 101) % 101, (noloop.1 % 103 + 103) % 103);
        if fin.0 < 50 && fin.1 < 51
        {
            qcnt[0] += 1;
        }
        else if fin.0 > 50 && fin.1 < 51
        {
            qcnt[1] += 1;
        }
        else if fin.0 < 50 && fin.1 > 51
        {
            qcnt[2] += 1;
        }
        else if fin.0 > 50 && fin.1 > 51
        {
            qcnt[3] += 1;
        }
        ln.clear();
        bc = stdin().read_line(&mut ln).unwrap();
    }
    println!("{}", qcnt.iter().product::<i32>());
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
