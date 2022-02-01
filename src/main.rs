use std::fs::File;
use std::io::Read;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct App0Marker {
    app0_marker: [u8; 2],
    length: [u8; 2],
    identifier: [u8; 5],
    version: [u8; 2],
    density_units: u8,
    xdensity: [u8; 2],
    ydensity: [u8; 2],
    xthumbnail: u8,
    ythumbnail: u8,
}
impl App0Marker {
    fn get_length(&self) -> u16 {
        (self.length[0] as u16 * 256 + self.length[1] as u16).into()
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct Header {
    soi: [u8; 2],
    app0: App0Marker,
}

fn read_file(fname: String) -> std::io::Result<()> {
    let mut f = File::open(fname)?;

    let mut header: Header = unsafe { std::mem::zeroed() };
    let header_size = std::mem::size_of::<Header>();

    unsafe {
        let config_slice =
            std::slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size);
        f.read_exact(config_slice).unwrap();
    }
    println!("Read structure: {:#?}", header);
    println!("App0 length {}", header.app0.get_length());
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
