use std::io::stdin;

type MachineType = [usize; 5];

fn combo(op: usize, machine: &MachineType) -> usize
{
    if op < 4
    {
        return op;
    }
    return machine[op - 3];
}

fn adv(machine: &mut MachineType, op: usize)
{
    machine[1] >>= combo(op, machine);
    machine[0] += 2;
}

fn bxl(machine: &mut MachineType, op: usize)
{
    machine[2] ^= op;
    machine[0] += 2;
}

fn bst(machine: &mut MachineType, op: usize)
{
    machine[2] = combo(op, machine) & 7;
    machine[0] += 2;
}

fn jnz(machine: &mut MachineType, op: usize)
{
    if machine[1] > 0
    {
        machine[0] = op;
    }
    else
    {
        machine[0] += 2;
    }
}

fn bxc(machine: &mut MachineType, _op: usize)
{
    machine[2] ^= machine[3];
    machine[0] += 2;
}

fn out(machine: &mut MachineType, op: usize)
{
    let v = combo(op, machine) & 7;
    if machine[4] > 0
    {
        print!(",{}", v);
    }
    else
    {
        print!("{}", v);
    }
    machine[4] += 1;
    machine[0] += 2;
}

fn bdv(machine: &mut MachineType, op: usize)
{
    machine[2] = machine[1] >> combo(op, machine);
    machine[0] += 2;
}

fn cdv(machine: &mut MachineType, op: usize)
{
    machine[3] = machine[1] >> combo(op, machine);
    machine[0] += 2;
}

fn main_rs(_args: Vec<String>) -> i32
{
    let mut computer: MachineType = [0; 5];
    let instructions = [adv, bxl, bst, jnz, bxc, out, bdv, cdv];
    let mut ln = String::new();
    let mut _q = stdin().read_line(&mut ln).unwrap();
    ln.pop();
    computer[1] = ln[12..].parse().unwrap();
    ln.clear();
    _q = stdin().read_line(&mut ln).unwrap();
    ln.pop();
    computer[2] = ln[12..].parse().unwrap();
    ln.clear();
    _q = stdin().read_line(&mut ln).unwrap();
    ln.pop();
    computer[3] = ln[12..].parse().unwrap();
    _q = stdin().read_line(&mut ln).unwrap();
    ln.clear();
    _q = stdin().read_line(&mut ln).unwrap();
    ln.pop();
    let program = ln[9..].split(",").map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
    while computer[0] < program.len()
    {
        let op = program[computer[0] + 1];
        instructions[program[computer[0]]](&mut computer, op);
    }
    println!("");
    0
}
fn main()
{
    let argv: Vec<String> = std::env::args().collect();
    std::process::exit(main_rs(argv));
}
