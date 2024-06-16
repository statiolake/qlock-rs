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

fn split_string_literal(s: &str) -> String {
    let s = s.trim_start_matches('"').trim_end_matches('"');
    let mut literals = vec![];
    let mut literal = String::new();
    let mut after_backslash = false;
    for c in s.chars() {
        if !after_backslash && literal.len() >= 10 {
            literals.push(literal);
            literal = String::new();
        }

        literal.push(c);

        after_backslash = c == '\\';
    }

    if !literal.is_empty() {
        literals.push(literal);
    }

    format!("concat!(\"{}\")", literals.join("\", \""))
}

fn compile(base: &str, out: &str) -> io::Result<()> {
    let source = read_to_string(base)?;
    let minified_source = minify(&source);
    let splitted_literal = split_string_literal(&format!("{minified_source:?}"));
    let embedded = source.replace("\"~\"", &splitted_literal);
    let final_source = minify(&embedded);
    write(out, final_source)?;
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
