use std::io::stdin;

fn solve(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> i32
{
    let det = a[0] * b[1] - a[1] * b[0];
    let adet = c[0] * b[1] - c[1] * b[0];
    let bdet = a[0] * c[1] - a[1] * c[0];
    if det == 0
    {
        if adet == 0
        {
            return std::cmp::min(c[0] / b[0], c[0] / a[0] * 3);
        }
        return 0;
    }
    if adet % det == 0 && bdet % det == 0
    {
        return(adet * 3 + bdet) / det;
    }
    return 0;
}

fn main_rs(args: Vec<String>) -> i32
{
    let mut tot = 0;
    let (mut dummystr, mut astr, mut bstr, mut targetstr) = (String::new(), String::new(), String::new(), String::new());
    let mut _tmp = stdin().read_line(&mut astr).unwrap();
    _tmp = stdin().read_line(&mut bstr).unwrap();
    _tmp = stdin().read_line(&mut targetstr).unwrap();
    let mut cnt: usize = 1;
    while cnt > 0
    {
        astr.pop();
        bstr.pop();
        targetstr.pop();
        let avec: Vec<i32> = astr[12..].split(", Y+").map(|x|x.parse().unwrap()).collect();
        let bvec: Vec<i32> = bstr[12..].split(", Y+").map(|x|x.parse().unwrap()).collect();
        let tvec: Vec<i32> = targetstr[9..].split(", Y=").map(|x|x.parse().unwrap()).collect();
        tot += solve(avec, bvec, tvec);
        astr.clear();
        bstr.clear();
        targetstr.clear();
        cnt = stdin().read_line(&mut dummystr).unwrap();
        _tmp = stdin().read_line(&mut astr).unwrap();
        _tmp = stdin().read_line(&mut bstr).unwrap();
        _tmp = stdin().read_line(&mut targetstr).unwrap();
    }
    println!("{}", tot);
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
