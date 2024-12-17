use std::io::stdin;

type MachineType = ([usize; 4], Vec<usize>);

fn combo(op: usize, machine: &MachineType) -> usize
{
    if op < 4
    {
        return op;
    }
    return machine.0[op - 3];
}

fn adv(machine: &mut MachineType, op: usize)
{
    machine.0[1] >>= combo(op, machine);
    machine.0[0] += 2;
}

fn bxl(machine: &mut MachineType, op: usize)
{
    machine.0[2] ^= op;
    machine.0[0] += 2;
}

fn bst(machine: &mut MachineType, op: usize)
{
    machine.0[2] = combo(op, machine) & 7;
    machine.0[0] += 2;
}

fn jnz(machine: &mut MachineType, op: usize)
{
    if machine.0[1] > 0
    {
        machine.0[0] = op;
    }
    else
    {
        machine.0[0] += 2;
    }
}

fn bxc(machine: &mut MachineType, _op: usize)
{
    machine.0[2] ^= machine.0[3];
    machine.0[0] += 2;
}

fn out(machine: &mut MachineType, op: usize)
{
    machine.1.push(combo(op, machine) & 7);
    machine.0[0] += 2;
}

fn bdv(machine: &mut MachineType, op: usize)
{
    machine.0[2] = machine.0[1] >> combo(op, machine);
    machine.0[0] += 2;
}

fn cdv(machine: &mut MachineType, op: usize)
{
    machine.0[3] = machine.0[1] >> combo(op, machine);
    machine.0[0] += 2;
}

fn runprog(c: &mut MachineType, program: &Vec<usize>)
{
    let instructions = [adv, bxl, bst, jnz, bxc, out, bdv, cdv];
    while c.0[0] < program.len()
    {
        let op = program[c.0[0] + 1];
        instructions[program[c.0[0]]](c, op);
    }
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut computer: MachineType = ([0; 4], Vec::new());
    let mut ln = String::new();
    let mut _q = stdin().read_line(&mut ln).unwrap();
    ln.pop();
    computer.0[1] = ln[12..].parse().unwrap();
    ln.clear();
    _q = stdin().read_line(&mut ln).unwrap();
    ln.pop();
    computer.0[2] = ln[12..].parse().unwrap();
    ln.clear();
    _q = stdin().read_line(&mut ln).unwrap();
    ln.pop();
    computer.0[3] = ln[12..].parse().unwrap();
    _q = stdin().read_line(&mut ln).unwrap();
    ln.clear();
    _q = stdin().read_line(&mut ln).unwrap();
    ln.pop();
    computer.0[1] = 2;
    let program = ln[9..].split(",").map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
    let mut cclone = computer.clone();
    let mut logradix = 1;
    runprog(&mut cclone, &program);
    while cclone.1.len() != 2
    {
        computer.0[1] <<= 1;
        logradix += 1;
        cclone = computer.clone();
        runprog(&mut cclone, &program);
    }
    computer.0[1] = 1 << ((program.len() - 1) * logradix);
    for i in (0..program.len()).rev()
    {
        loop
        {
            cclone = computer.clone();
            runprog(&mut cclone, &program);
            if cclone.1[i..] == program[i..]
            {
                break;
            }
            computer.0[1] += 1 << (i * logradix);
        }
    }
    println!("{}", computer.0[1]);
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
