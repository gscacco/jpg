use std::fs::File;
use std::io::{Read, Seek};
use std::mem::{size_of, zeroed};

//https://en.wikipedia.org/wiki/JPEG_File_Interchange_Format
//Exif: https://www.media.mit.edu/pia/Research/deepview/exif.html

fn bigendian16(arr: [u8; 2]) -> u16 {
    (arr[0] as u16 * 256 + arr[1] as u16).into()
}
fn _bigendian32(arr: [u8; 4]) -> u32 {
    let iter = arr.iter();

    let mut res: u32 = 0;
    for (i, v) in iter.enumerate() {
        res += *v as u32 * 256_u32.pow(i as u32);
    }
    res
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
    ssss: [u8; 2],
    exif: [u8; 4],
    zero: [u8; 2],
}
impl App1Marker {
    fn get_size(&self) -> u16 {
        bigendian16(self.ssss)
    }
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
                println!("Version {}.{:#02}", app0.version[0], app0.version[1]);
                println!("Density units {}", app0.density_units);
                let app0_rst = app0.get_length() - 16;
                println!("Length {}", app0_rst);
                if app0_rst > 0 {
                    f.seek(std::io::SeekFrom::Current(app0_rst.into()))?;
                }
            }
            (0xFF, 0xE1) => {
                println!("EXIF");
                let mut app1: App1Marker = unsafe { zeroed() };
                read_struct(&f, &mut app1 as *mut _ as *mut u8, size_of::<App1Marker>())?;
                println!("Size {}", app1.get_size());
                println!("Identifier {}", std::str::from_utf8(&app1.exif).unwrap());
            }
            _ => {
                println!("Unknown block {:?}", h.header);
                go = false;
            }
        }
    }

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
