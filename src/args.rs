use std::env;

#[derive(Debug)]
pub struct Args {
    pub filename: String,
    pub breakpoint: Option<u16>,
}

pub fn read_args() -> Args {
    let mut args = env::args();

    let prog_name = args.next().unwrap();

    let mut breakpoint = None;
    let mut filename = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--break" => {
                let line: u16 = args
                    .next()
                    .expect("Expected a number after --break")
                    .parse::<u16>()
                    .expect("Failed to parse breakpoint");
                breakpoint = Some(line);
            }
            "--help" => {
                print_help(&prog_name);
                std::process::exit(0);
            }
            _ => {
                if filename.is_some() {
                    println!("filename already set");
                    print_help(&prog_name);
                    std::process::exit(1)
                }
                filename = Some(arg.to_string())
            }
        }
    }

    if let Some(fname) = filename {
        Args {
            filename: fname,
            breakpoint: breakpoint,
        }
    } else {
        println!("filename not set");
        print_help(&prog_name);
        std::process::exit(1)
    }
}

fn print_help(name: &str) {
    println!("Usage: {name} [--break line] <filename>");
    println!();
    println!("Options:");
    println!("  --break <line>    Set a breakpoint at the given line number");
    println!("  -h, --help        Print this help message");
}
