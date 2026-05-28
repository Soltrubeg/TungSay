
/*
I found this divine creation in https://emojicombos.com/tung-tung-tung-sahur, no fucking clue
how to get the exact link though
*/
const ASCII: &str = r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣄⣀⣀⡀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣄
⣦⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⠛⠛⠿⠿⢿⣿⣦⣠⣄⡀⣠⠿⣿⣿⣿⣿⠟⠛⠛⠛⠛⢿⣿⣿⡆
⠀⠀⠀⠀⠀⠀⠀⠉⠙⠋⠀⣀⠀⠈⢻⡏⠀⢀⢤⣤⣤⡹⣿⣿⡇⠀⠀⠀⠀⠙
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠞⠺⣿⡆⢸⣷⠀⡼⠉⠙⢻⣟⠸⣿⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡀⠀⣼⡗⣾⣿⣷⡿⣀⣀⡾⣻⣾⣿⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣄⣨⣵⣾⣿⣿⣿⣿⣶⣶⣿⣿⣿⡿⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠠⡛⠿⠟⠋⠞⠻⠿⠿⢎⡉⠉⠉⠉
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢦⠀⢼⡄⠀⢀⣴⣿⡿⠂
⢰⣄⡀⠀⠀⠀⠀⠀⠀⠀⠈⣇⠀⠀⠀⠠⠶⠖⣪⣿⣷
⣸⣿⣷⣤⡀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⢀⣤⣶⣿⡿⠛⣀
⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⢸⡇⠀⠀⠛⠛⠋⢉⣤⣾⣿⣷
⣿⣿⣿⣿⣿⣿⣿⣶⠀⠀⠀⢸⡇⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣶⡄
⣿⣿⣿⣿⣿⣿⣿⣿⣇⠀⠀⡎⡇⠀⢠⣿⣿⣿⣿⣿⣿⡿⠈⣿⡇
⢿⣿⣿⣿⣿⣿⣿⣿⣿⡷⠀⡇⣿⠀⢸⣿⣿⣿⣿⣿⣿⡇⠀⢻⣷
⣽⣿⣿⣿⣿⣿⣿⣿⣿⡟⢄⡇⢹⠀⠈⢿⣿⣿⣿⣿⣿⣿⠀⢨⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣼⡁⠸⠀⠀⣼⣿⣿⣿⣿⣿⣿⠀⣾⡏
⣹⣯⣭⣭⣄⣸⣅⡀⠀⠀⢸⡇⠘⠀⠀⢻⣿⣿⣿⣿⣿⡿⢠⣿⠁
⢹⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣧⠀⠀⠀⠀⣿⣿⣿⣿⣿⣧⣼⡏
⢸⣿⣿⣿⣿⣿⣿⡇⠀⠀⢀⠙⡎⠀⠀⠀⣿⣿⣿⣿⣿⠋⡽⠃
⠸⠿⠿⠿⠿⠿⠻⠃⠀⠀⢀⣵⠇⠀⠀⠀⠀⢾⣿⣿⠃⠔⠁⠀⠁⠛⠒⠷⠒⠒
"#;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (lines, length) = wrap(&*args.join(" "), 50);
    let padding = length / 2 + 2;
    let bars_length = length + 4;
    let body = {
        let mut body: Vec<String> = vec![];
        for line in lines {
            let padding = length - line.chars().count();
            let left = padding / 2;
            let right = padding - left;
            let mut part: String = String::new();
            part.push_str("< ");
            part.push_str(&*" ".repeat(left));
            part.push_str(&line);
            part.push_str(&*" ".repeat(right));
            part.push_str(" >");
            body.push(part);
        }
        body.join("\n")
    };
    print!(
        "{}\n{body}\n{}\n{}\\\n{}\\\n",
        "_".repeat(bars_length),
        "-".repeat(bars_length),
        " ".repeat(padding),
        " ".repeat(padding + 1)
    );
    println!("{}", indent(ASCII.trim(), padding));
}

fn indent(s: &str, spaces: usize) -> String {
    s.lines()
        .map(|line| format!("{:>width$}{line}", "", width = spaces))
        .collect::<Vec<_>>()
        .join("\n")
}

fn wrap(text: &str, width: usize) -> (Vec<String>, usize) {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    let mut length = 0;

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.chars().count() + 1 + word.chars().count() <= width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line.clone());
            current_line = word.to_string();
        }
        length = length.max(current_line.chars().count());
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    (lines, length)
}
