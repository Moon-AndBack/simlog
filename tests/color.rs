use colored::{ColoredString, Colorize};

#[test]
fn co_test() {
    let a = "aaa".red();
    let b = "bbb".black();
    let c = "ccc".blue();
    let d = "ddd".yellow();
    let e = "eee".green();

    let x = std::fmt::format(format_args!("{}{}{}{}{}", a, b, c, d, e));
    println!("{}", &x);
    eprintln!("{}", &x);
}
