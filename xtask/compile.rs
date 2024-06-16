use std::{
    fs::{read_to_string, write},
    io,
};

const SYMBOLS: &[u8] = &[
    b',', b':', b'!', b'+', b'-', b'*', b'/', b'%', b'>', b'=', b'|', b'&', b'[', b']', b'{', b'}',
    b'(', b')', b';',
];

fn minify(s: &str) -> String {
    let tokens: Vec<_> = s.split_whitespace().collect();
    let total_width = tokens.iter().map(|t| t.len()).sum::<usize>();
    let single_line_width = total_width / 7;

    let mut result = String::new();
    let mut current_line_width = 0;
    for tok in tokens {
        if current_line_width != 0
            && !SYMBOLS.contains(&result.as_bytes()[result.len() - 1])
            && !SYMBOLS.contains(&tok.as_bytes()[0])
        {
            result.push(' ');
            current_line_width += 1;
        }

        result.push_str(tok);
        current_line_width += tok.len();

        if current_line_width >= single_line_width {
            result.push('\n');
            current_line_width = 0;
        }
    }

    result
}

fn compile(base: &str, out: &str) -> io::Result<()> {
    let s = read_to_string(base)?;
    let s = minify(&s);
    let ss = format!("{s:?}");
    let s = s.replace('~', &ss[1..ss.len() - 1]);
    write(out, s)?;
    Ok(())
}

pub fn main() {
    let mut failed = false;
    for (base, out) in [
        ("src/quine_base.rs", "src/quine.rs"),
        ("src/qlock_base.rs", "src/qlock.rs"),
    ] {
        failed = compile(base, out).is_err() || failed;
    }

    if failed {
        panic!("Failed to compile one of the files");
    }
}
