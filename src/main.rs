mod io;
mod othello;
mod wthor;

fn main() {
    let (boards, indexes) = crate::wthor::parse((2010..=2020).map(|year| format!("wthor/WTH_{}.wtb", year))).unwrap();
    println!("{}", boards.into_iter().map(|vec| vec.into_iter().map(|vec| vec.len()).sum::<usize>()).sum::<usize>());
    println!("{}", indexes.len());
}
