use sponk::prelude::*;

fn main() {
    println!("sponk");
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    })
}

fn run() -> Result<()> {
    let mut s = Scanner::new("57984023098753490873254798053247098");
    let mut tokens = Vec::new();

    loop {
        let t = s.next_token()?;
        if t.kind() == TokenKind::EOF {
            break;
        } else {
            tokens.push(t);
        }
    }

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
