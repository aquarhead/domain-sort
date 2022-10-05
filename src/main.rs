use anyhow::Result;
use std::io;

fn main() -> Result<()> {
  let mut domains = Vec::new();
  loop {
    let mut line = String::new();
    if io::stdin().read_line(&mut line)? > 0 {
      domains.push(line.trim().to_string());
    } else {
      break;
    }
  }

  domains.sort_by_key(|d| d.split('.').rev().collect::<Vec<_>>().join("."));

  domains.iter().for_each(|d| println!("{}", d));

  Ok(())
}
