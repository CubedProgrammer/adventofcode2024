use std::io::stdin;

fn main()
{
    let lb = 40 as char;
    let rb = 41 as char;
    let name = "mul";
    let mut ln = String::new();
    let mut bc = stdin().read_line(&mut ln).unwrap();
    let mut tot: u64 = 0;
    while bc > 0
    {
        let mut ind = ln.find(name);
        let mut slice = ln.as_str();
        while ind.is_some()
        {
            slice = &slice[ind.unwrap()+3..];
            let lind = slice.find(lb);
            let cind = slice.find(',');
            let rind = slice.find(rb);
            if !(lind.is_none() || cind.is_none() || rind.is_none())
            {
                let (lu, cu, ru) = (lind.unwrap(), cind.unwrap(), rind.unwrap());
                if lu == 0 && lu < cu && cu < ru
                {
                    let (x, y) = (slice[1..cu].parse::<u64>(), slice[cu+1..ru].parse::<u64>());
                    match x
                    {
                        Ok(u) => match y
                        {
                            Ok(v) => tot += u * v,
                            Err(_) => {}
                        }
                        Err(_) => {}
                    }
                }
            }
            ind = slice.find(name);
        }
        ln.clear();
        bc = stdin().read_line(&mut ln).unwrap();
    }
    println!("{}", tot);
}
