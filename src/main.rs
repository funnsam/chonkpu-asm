mod lexer;
mod parser;
mod postproc;
mod bingen;

use logos::Logos;

fn main() {
    let src_loc = std::env::args().nth(1).expect("expected filename as 1st arg");
    let src = std::fs::read_to_string(&src_loc).expect("failed to read file");

    let b = comp(&src);
    let mut b = if let Ok(b) = b {
        b
    } else {
        let s = b.unwrap_err();
        println!("Error at: {}", &src[s]);
        std::process::exit(1);
    };

    for s in b.iter() {
        println!("{s:03x}");
    }

    if b.len() > 256 {
        println!("Binary too large!");
        std::process::exit(1);
    }

    b.resize(256, 0x400);

    let b = b.into_iter().map(|a| a.to_be_bytes()).flatten().collect::<Vec<u8>>();
    std::fs::write(format!("{src_loc}.bin"), b).unwrap();
}

fn comp(s: &str) -> Result<Vec<u16>, logos::Span> {
    let l = lexer::Token::lexer(s);
    let p = parser::parse(l)?;
    let p = postproc::post_proc(p)?;
    Ok(bingen::generate(p))
}
