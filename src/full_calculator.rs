// this project is meant to act as a calculator
use std::io;

pub(crate) fn addition() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: i32 = a.trim().parse().unwrap();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let b: i32 = b.trim().parse().unwrap();

    let sum: i32 = a + b;
    println!("{}", sum);
}

pub(crate) fn subtraction() {
    let mut c = String::new();
    io::stdin().read_line(&mut c).unwrap();
    let c: i32 = c.trim().parse().unwrap();

    let mut d = String::new();
    io::stdin().read_line(&mut d).unwrap();
    let d: i32 = d.trim().parse().unwrap();

    let subtraction: i32 = c - d;
    println!("{}", subtraction);
}
pub(crate) fn multiplication(){
    let mut e = String::new();
    io::stdin().read_line(&mut e).unwrap();
    let e: i32 = e.trim().parse().unwrap();

    let mut f = String::new();
    io::stdin().read_line(&mut f).unwrap();
    let f: i32 = f.trim().parse().unwrap();
    let multiplication: i32 = e * f;
    println!("{}", multiplication);

}
pub(crate) fn division(){
    let mut g = String::new();
    io::stdin().read_line(&mut g).unwrap();
    let g: i32 = g.trim().parse().unwrap();

    let mut h = String::new();
    io::stdin().read_line(&mut h).unwrap();
    let h: i32 = h.trim().parse().unwrap();
    let division: i32= g / h;
    println!("{}", division);

}