use chrono::Timelike;

fn px(d: Option<&usize>, i: usize, j: usize) -> bool {
    if d.is_none() || i >= 5 || j >= 3 {
        return false;
    }

    ([
        31599, 19812, 14479, 31207, 23524, 29411, 29679, 30866, 31727, 31719, 1040,
    ][*d.unwrap()]
        >> ((4 - i) * 3 + j))
        & 1
        != 0
}

fn pq(s: &[usize]) {
    print!("\x1B[1;1H");

    let (mut i, mut j) = (0, 0);
    let mut pc = |c| {
        if c == '\n' {
            j = 0;
            i += 1;
            println!();
        } else {
            if px(s.get(j / 4), i, j % 4) {
                print!("\x1b[31m{c}\x1b[0m");
            } else {
                print!("{c}");
            }
            j += 1;
        }
    };

    let s = "~";
    s.chars().for_each(|c| {
        if c as u8 == 0x7e {
            s.chars().for_each(|c| match c {
                '\n' => {
                    pc('\\');
                    pc('n');
                }
                '"' => {
                    pc('\\');
                    pc('"');
                }
                '\\' => {
                    pc('\\');
                    pc('\\');
                }
                _ => {
                    pc(c);
                }
            });
        } else {
            pc(c)
        }
    })
}

fn main() {
    print!("\x1B[2J");
    loop {
        let d = chrono::Local::now();
        let (h, m, s) = (d.hour() as usize, d.minute() as usize, d.second() as usize);
        pq(&[h / 10, h % 10, 10, m / 10, m % 10, 10, s / 10, s % 10]);
    }
}
