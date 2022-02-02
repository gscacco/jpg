#[allow(dead_code)]
pub mod data {
    pub fn bigendian16(arr: [u8; 2]) -> u16 {
        (arr[0] as u16 * 256 + arr[1] as u16).into()
    }
    pub fn _bigendian32(arr: [u8; 4]) -> u32 {
        let iter = arr.iter();

        let mut res: u32 = 0;
        for (i, v) in iter.enumerate() {
            res += *v as u32 * 256_u32.pow(i as u32);
        }
        res
    }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct GenericHeader {
        pub header: [u8; 2],
    }
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct App0Marker {
        pub length: [u8; 2],
        pub identifier: [u8; 5],
        pub version: [u8; 2],
        pub density_units: u8,
        pub xdensity: [u8; 2],
        pub ydensity: [u8; 2],
        pub xthumbnail: u8,
        pub ythumbnail: u8,
    }
    #[derive(Debug)]
    pub enum ByteOrder {
        Big,
        Little,
    }
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct TiffHeader {
        pub byte_order: [u8; 2],
        pub magic_number: [u8; 2],
        pub offset: [u8; 4],
    }
    impl TiffHeader {
        pub fn get_byte_order(&self) -> Result<ByteOrder, &str> {
            match (self.byte_order[0], self.byte_order[1]) {
                (0x49, 0x49) => Ok(ByteOrder::Little),
                (0x4D, 0x4D) => Ok(ByteOrder::Big),
                _ => Err("Wrong byte order"),
            }
        }
    }
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct App1Marker {
        pub ssss: [u8; 2],
        pub exif: [u8; 4],
        pub zero: [u8; 2],
        pub tiff_header: TiffHeader,
    }
    impl App1Marker {
        pub fn get_size(&self) -> u16 {
            bigendian16(self.ssss)
        }
    }
    impl App0Marker {
        pub fn get_length(&self) -> u16 {
            bigendian16(self.length)
        }
        pub fn get_xdensity(&self) -> u16 {
            bigendian16(self.xdensity)
        }
        pub fn get_ydensity(&self) -> u16 {
            bigendian16(self.ydensity)
        }
    }
}
