use std::io;

fn main()
{
    let mut ln = String::new();
    let mut bc = io::stdin().read_line(&mut ln).unwrap();
    let mut cnt = 0;
    while bc > 0
    {
        ln.pop();
        let arr: Vec<i32> = ln.split(" ").map(|str|str.parse().unwrap()).collect();
        if (arr.windows(2).all(|x|x[0] > x[1]) || arr.windows(2).all(|x|x[0] < x[1])) && arr.windows(2).all(|x|(x[1] - x[0]).abs() >= 1 && (x[1] - x[0]).abs() <= 3)
        {
            cnt += 1;
        }
        ln.clear();
        bc = io::stdin().read_line(&mut ln).unwrap();
    }
    println!("{}", cnt);
}
