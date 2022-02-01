use std::fs::File;
use std::io::{Read, Seek};

//https://en.wikipedia.org/wiki/JPEG_File_Interchange_Format

fn bigendian16(arr: [u8; 2]) -> u16 {
    (arr[0] as u16 * 256 + arr[1] as u16).into()
}

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

struct App1Marker {
    ssss: [u8; 4],
}

impl App0Marker {
    fn get_length(&self) -> u16 {
        bigendian16(self.length)
    }
    fn get_xdensity(&self) -> u16 {
        bigendian16(self.xdensity)
    }
    fn get_ydensity(&self) -> u16 {
        bigendian16(self.ydensity)
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct Header {
    soi: [u8; 2],
    app0: App0Marker,
}
fn read_struct(mut f: &File, structure: *mut u8, size: usize) -> std::io::Result<()> {
    unsafe {
        let slice = std::slice::from_raw_parts_mut(structure, size);
        f.read_exact(slice).unwrap();
    }
    Ok(())
}
fn read_file(fname: String) -> std::io::Result<()> {
    let mut f = File::open(fname)?;

    let mut header: Header = unsafe { std::mem::zeroed() };
    let header_size = std::mem::size_of::<Header>();
    
    read_struct(&f, &mut header as *mut _ as *mut u8, header_size)?;
    
    println!("Read structure: {:#?}", header);
    println!("App0 length {}", header.app0.get_length());
    println!("App0 xdensity {}", header.app0.get_xdensity());
    println!("App0 ydensity {}", header.app0.get_ydensity());
    let app0_rst = header.app0.get_length() - 16;
    if app0_rst > 0 {
        f.seek(std::io::SeekFrom::Current(app0_rst.into()))?;
    }
    println!("{}", app0_rst);
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
