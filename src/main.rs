fn find_solution(n: usize) -> String {
    if n % 15 == 0 {
        "CracklePop".to_string()
    } else if n % 5 == 0 {
        "Pop".to_string()
    } else if n % 3 == 0 {
        "Crackle".to_string()
    } else {
        n.to_string()
    }
}
fn main() {
    for n in 1..101 {
        println!("{}", find_solution(n));
    }
}
