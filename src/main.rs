use std::fs::File;
use std::io::Read;
fn main() -> std::io::Result<()> {
    let fname = std::env::args()
        .skip(1)
        .next()
        .expect("You must provide file name");
    println!("Analizing <{:?}>", fname);
    let mut f = File::open(fname)?;
    let mut buf = [0u8; 2];
    f.read(&mut buf)?;
    println!("{:#02X} {:#02X}", buf[0], buf[1]);
    Ok(())
}
