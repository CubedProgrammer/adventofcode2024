use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::io::stdin;

fn verify(x: &Vec<String>, y: &Vec<String>, z: &Vec<String>, up: &mut HashMap<[String; 3], String>, dn: &mut HashMap<String, [String; 3]>) -> Vec<String>
{
    let mut bad: Vec<String> = Vec::new();
    let mut initial_xor: [String; 3] = [x[0].clone(), String::from("XOR"), y[0].clone()];
    let mut initial_and: [String; 3] = [x[0].clone(), String::from("AND"), y[0].clone()];
    let z00 = up[&initial_xor].clone();
    let mut carry: String = up[&initial_and].clone();
    if z00 != z[0]
    {
        bad.push(z[0].clone());
        bad.push(z00.clone());
        *dn.get_mut(&z00).unwrap() = dn[&z[0]].clone();
        *dn.get_mut(&z[0]).unwrap() = initial_xor.clone();
        *up.get_mut(&dn[&z00]).unwrap() = up[&initial_xor].clone();
        *up.get_mut(&initial_xor).unwrap() = z[0].clone();
    }
    for ((xbit, ybit), zbit) in x.iter().zip(y.iter()).zip(z.iter()).skip(1)
    {
        initial_xor[0] = xbit.clone();
        initial_xor[2] = ybit.clone();
        initial_and[0] = xbit.clone();
        initial_and[2] = ybit.clone();
        let q = up[&initial_xor].clone();
        let qcarrysmall = min(&q, &carry);
        let qcarrylarge = max(&q, &carry);
        let second_xor: [String; 3] = [qcarrysmall.clone(), String::from("XOR"), qcarrylarge.clone()];
        let second_and: [String; 3] = [qcarrysmall.clone(), String::from("AND"), qcarrylarge.clone()];
        let s = up.get(&second_xor);
        let t = match s
        {
            Some(sv) => {
                if sv != zbit
                {
                    bad.push(sv.clone());
                    bad.push(zbit.clone());
                    *dn.get_mut(sv).unwrap() = dn[zbit].clone();
                    *dn.get_mut(zbit).unwrap() = second_xor.clone();
                    let old = &dn[sv];
                    *up.get_mut(old).unwrap() = up[&second_xor].clone();
                    *up.get_mut(&second_xor).unwrap() = zbit.clone();
                }
                up[&second_and].clone()
            },
            None => {
                let mut zeq = dn[zbit].clone();
                if zeq[0] != *qcarrysmall
                {
                    bad.push(zeq[0].clone());
                    bad.push(qcarrysmall.clone());
                    let dtmp = dn[qcarrysmall].clone();
                    *dn.get_mut(qcarrysmall).unwrap() = dn[&zeq[0]].clone();
                    *dn.get_mut(&zeq[0]).unwrap() = dtmp;
                    let utmp = up[&dn[&zeq[0]]].clone();
                    *up.get_mut(&dn[&zeq[0]]).unwrap() = up[&dn[qcarrysmall]].clone();
                    *up.get_mut(&dn[qcarrysmall]).unwrap() = utmp;
                }
                if zeq[2] != *qcarrylarge
                {
                    bad.push(zeq[2].clone());
                    bad.push(qcarrylarge.clone());
                    let dtmp = dn[qcarrylarge].clone();
                    *dn.get_mut(qcarrylarge).unwrap() = dn[&zeq[2]].clone();
                    *dn.get_mut(&zeq[2]).unwrap() = dtmp;
                    let utmp = up[&dn[&zeq[2]]].clone();
                    *up.get_mut(&dn[&zeq[2]]).unwrap() = up[&dn[qcarrylarge]].clone();
                    *up.get_mut(&dn[qcarrylarge]).unwrap() = utmp;
                }
                zeq[1] = second_and[1].clone();
                up[&zeq].clone()
            }
        };
        let r = up[&initial_and].clone();
        let (trsmall, trlarge) = (min(&t, &r), max(&t, &r));
        let final_or: [String; 3] = [trsmall.clone(), String::from("OR"), trlarge.clone()];
        match up.get(&final_or)
        {
            Some(val) => carry = val.clone(),
            None => {}
        }
    }
    bad.sort();
    bad
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut dngraph: HashMap<String, [String; 3]> = HashMap::new();
    let mut upgraph: HashMap<[String; 3], String> = HashMap::new();
    let mut xval: Vec<String> = Vec::new();
    let mut yval: Vec<String> = Vec::new();
    let mut zval: Vec<String> = Vec::new();
    let mut ln = String::new();
    let mut bc = stdin().read_line(&mut ln).unwrap();
    while bc > 1
    {
        ln.pop();
        if ln.starts_with("x")
        {
            xval.push(ln[0..3].to_string());
        }
        else
        {
            yval.push(ln[0..3].to_string());
        }
        ln.clear();
        bc = stdin().read_line(&mut ln).unwrap();
    }
    for oln in stdin().lines()
    {
        let mut eq = oln.unwrap().split_ascii_whitespace().map(|x|String::from(x)).collect::<Vec<String>>();
        if eq[4].starts_with("z")
        {
            zval.push(eq[4].clone());
        }
        if eq[0] > eq[2]
        {
            eq.swap(0, 2);
        }
        let expr = [eq[0].clone(), eq[1].clone(), eq[2].clone()];
        upgraph.insert(expr.clone(), eq[4].clone());
        dngraph.insert(eq[4].clone(), expr);
    }
    xval.sort();
    yval.sort();
    zval.sort();
    let res = verify(&xval, &yval, &zval, &mut upgraph, &mut dngraph);
    let mut notfirst: bool = false;
    for i in res
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
