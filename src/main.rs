use std::fs::File;
use std::io::Read;

fn read_file(fname: String) -> std::io::Result<()> {
    let mut f = File::open(fname)?;

    let mut soi = [0u8; 2];
    f.read_exact(&mut soi)?;
    assert_eq!([0xFF, 0xD8], soi);

    let mut app0_marker = [0u8; 2];
    f.read_exact(&mut app0_marker)?;
    assert_eq!([0xFF, 0xE0], app0_marker);

    Ok(())
}
fn main() -> std::io::Result<()> {
    let fname = std::env::args()
        .skip(1)
        .next()
        .expect("You must provide file name");
    println!("Analizing <{:?}>", fname);
    read_file(fname)?;
    Ok(())
}
