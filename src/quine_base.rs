fn main() {
    let s = "~";
    for c in s.chars() {
        match c as u8 {
            0x7e => {
                let s = format!("{s:?}");
                print!("{}", &s[1..s.len() - 1]);
            }
            _ => print!("{}", c),
        }
    }
}
