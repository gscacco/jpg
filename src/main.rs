use std::fs::File;
use std::io::{Read, Seek};
use std::mem::{size_of, zeroed};
mod data;

//https://en.wikipedia.org/wiki/JPEG_File_Interchange_Format
//Exif: https://www.media.mit.edu/pia/Research/deepview/exif.html
//https://docs.fileformat.com/image/exif/

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
        let mut h: data::GenericHeader = unsafe { zeroed() };
        read_struct(&f, &mut h as *mut _ as *mut u8, size_of::<data::GenericHeader>())?;

        match (h.header[0], h.header[1]) {
            (0xFF, 0xD8) => println!("Start Of Image"),
            (0xFF, 0xE0) => {
                println!("APP0");
                let mut app0: data::App0Marker = unsafe { zeroed() };
                read_struct(&f, &mut app0 as *mut _ as *mut u8, size_of::<data::App0Marker>())?;

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
                let mut app1: data::App1Marker = unsafe { zeroed() };
                read_struct(&f, &mut app1 as *mut _ as *mut u8, size_of::<data::App1Marker>())?;
                println!("Size {}", app1.get_size());
                println!("Identifier {}", std::str::from_utf8(&app1.exif).unwrap());
                println!("tiff header {:?}", app1.tiff_header);
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
