use std::fs::File;
use std::io::{Read, Seek};
use std::mem::{size_of, zeroed};
#[path = "data.rs"]
mod data;
use data::data::{App0Marker, App1Marker, GenericHeader};

fn read_struct(mut f: &File, structure: *mut u8, size: usize) -> std::io::Result<()> {
    unsafe {
        let slice = std::slice::from_raw_parts_mut(structure, size);
        f.read_exact(slice).unwrap();
    }
    Ok(())
}

fn read_app0(mut f: &File) -> std::io::Result<()> {
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
    Ok(())
}
fn read_exif(f: &File) -> std::io::Result<()> {
    println!("EXIF");
    let mut app1: App1Marker = unsafe { zeroed() };
    read_struct(&f, &mut app1 as *mut _ as *mut u8, size_of::<App1Marker>())?;
    println!("Size {}", app1.get_size());
    println!("Identifier {}", std::str::from_utf8(&app1.exif).unwrap());
    println!("tiff header {:?}", app1.tiff_header);
    println!("Byte order: {:?}", app1.tiff_header.get_byte_order());
    Ok(())
}

pub fn read_file(fname: String) -> std::io::Result<()> {
    let f = File::open(fname)?;

    let mut go = true;
    while go {
        let mut h: data::data::GenericHeader = unsafe { zeroed() };
        read_struct(&f, &mut h as *mut _ as *mut u8, size_of::<GenericHeader>())?;

        match (h.header[0], h.header[1]) {
            (0xFF, 0xD8) => println!("Start Of Image"),
            (0xFF, 0xE0) => read_app0(&f)?,
            (0xFF, 0xE1) => read_exif(&f)?,
            _ => {
                println!("Unknown block {:?}", h.header);
                go = false;
            }
        }
    }

    Ok(())
}
