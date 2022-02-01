use std::fs::File;
use std::io::{Read, Seek};
use std::mem::{size_of, zeroed};

//https://en.wikipedia.org/wiki/JPEG_File_Interchange_Format
//Exif: https://www.media.mit.edu/pia/Research/deepview/exif.html

fn bigendian16(arr: [u8; 2]) -> u16 {
    (arr[0] as u16 * 256 + arr[1] as u16).into()
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct GenericHeader {
    header: [u8; 2],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct App0Marker {
    length: [u8; 2],
    identifier: [u8; 5],
    version: [u8; 2],
    density_units: u8,
    xdensity: [u8; 2],
    ydensity: [u8; 2],
    xthumbnail: u8,
    ythumbnail: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
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

    let mut go = true;
    while go {
        let mut h: GenericHeader = unsafe { zeroed() };
        read_struct(&f, &mut h as *mut _ as *mut u8, size_of::<GenericHeader>())?;

        match (h.header[0], h.header[1]) {
            (0xFF, 0xD8) => println!("Start Of Image"),
            (0xFF, 0xE0) => {
                println!("APP0");
                let mut app0: App0Marker = unsafe { zeroed() };
                read_struct(&f, &mut app0 as *mut _ as *mut u8, size_of::<App0Marker>())?;

                println!(
                    "Identifier {}",
                    std::str::from_utf8(&app0.identifier[0..4]).unwrap()
                );
                println!("Density {} x {}", app0.get_xdensity(), app0.get_ydensity());
                println!("Length {}", app0.get_length() - 16);
            }
            _ => {
                println!("Unknown block {:?}", h.header);
                go = false;
            }
        }
    }

    /* let mut header: Header = unsafe { std::mem::zeroed() };

    read_struct(&f, &mut header as *mut _ as *mut u8, size_of::<Header>())?;

    println!("Read structure: {:#?}", header);
    println!("App0 length {}", header.app0.get_length());
    println!("App0 xdensity {}", header.app0.get_xdensity());
    println!("App0 ydensity {}", header.app0.get_ydensity());
    let app0_rst = header.app0.get_length() - 16;
    if app0_rst > 0 {
        f.seek(std::io::SeekFrom::Current(app0_rst.into()))?;
    }
    println!("{}", app0_rst); */
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
