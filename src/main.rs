use argh::FromArgs;

#[derive(FromArgs)]
/// A utility to join lines in a file.
struct Joined {
    /// target file
    #[argh(positional)]
    file: String,
    /// join separator
    #[argh(option, short = 's', default = "String::from(\" \")")]
    sep: String,
}

fn main() {
    let args: Joined = argh::from_env();
    let content = std::fs::read(args.file).expect("Reading file failed");
    println!(
        "{}",
        String::from_utf8_lossy(&content)
            .split('\n')
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>()
            .join(&args.sep)
    );
}
