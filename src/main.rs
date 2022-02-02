mod data;
mod utils;

fn main() -> std::io::Result<()> {
    let fname = std::env::args()
        .skip(1)
        .next()
        .expect("You must provide file name");
    println!("Analizing <{:?}>", fname);
    utils::read_file(fname)?;
    Ok(())
}
