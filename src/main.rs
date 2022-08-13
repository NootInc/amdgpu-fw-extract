use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
struct FwLegacyBinDesc {
    _ver: u32,
    off: u32,
    size: u32,
}

impl FwLegacyBinDesc {
    pub fn from_reader<T>(r: &mut T) -> Self
    where
        T: std::io::Read,
    {
        Self {
            _ver: r.read_u32::<LittleEndian>().unwrap(),
            off: r.read_u32::<LittleEndian>().unwrap(),
            size: r.read_u32::<LittleEndian>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct PspTaFw {
    pub xgmi: FwLegacyBinDesc,
    pub ras: FwLegacyBinDesc,
    pub hdcp: FwLegacyBinDesc,
    pub dtm: FwLegacyBinDesc,
    pub secure_display: FwLegacyBinDesc,
}

impl PspTaFw {
    pub fn from_reader<T>(r: &mut T) -> Self
    where
        T: std::io::Read,
    {
        Self {
            xgmi: FwLegacyBinDesc::from_reader(r),
            ras: FwLegacyBinDesc::from_reader(r),
            hdcp: FwLegacyBinDesc::from_reader(r),
            dtm: FwLegacyBinDesc::from_reader(r),
            secure_display: FwLegacyBinDesc::from_reader(r),
        }
    }
}

fn main() {
    let type_ = std::env::args().nth(1).unwrap();
    let fw = std::env::args().nth(2).unwrap();
    let output = std::env::args().nth(3).unwrap();
    let fw = std::fs::read(fw).unwrap();
    let mut c = std::io::Cursor::new(fw);
    c.set_position(20);
    let ucode_size = c.read_u32::<LittleEndian>().unwrap();
    let ucode_off = c.read_u32::<LittleEndian>().unwrap();
    c.seek(SeekFrom::Current(4)).unwrap();
    match type_.to_lowercase().as_str() {
        "psp_xgmi" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            println!("{:#x?}", ta_fw);
            if ta_fw.xgmi.size == 0 {
                eprintln!("XGMI size 0!");
                return;
            }
            c.set_position(ucode_off as u64 + ta_fw.xgmi.off as u64);
            let mut buf = Vec::new();
            buf.resize(ta_fw.xgmi.size as usize, 0);
            c.read_exact(buf.as_mut_slice()).unwrap();
            println!("Saving PSP XGMI to {}", output);
            std::fs::write(output, buf).unwrap();
        }
        "psp_ras" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            println!("{:#x?}", ta_fw);
            if ta_fw.ras.size == 0 {
                eprintln!("RAS size 0!");
                return;
            }
            c.set_position(ucode_off as u64 + ta_fw.ras.off as u64);
            let mut buf = Vec::new();
            buf.resize(ta_fw.ras.size as usize, 0);
            c.read_exact(buf.as_mut_slice()).unwrap();
            println!("Saving PSP RAS to {}", output);
            std::fs::write(output, buf).unwrap();
        }
        "psp_hdcp" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            println!("{:#x?}", ta_fw);
            if ta_fw.hdcp.size == 0 {
                eprintln!("HDCP size 0!");
                return;
            }
            c.set_position(ucode_off as u64 + ta_fw.hdcp.off as u64);
            let mut buf = Vec::new();
            buf.resize(ta_fw.hdcp.size as usize, 0);
            c.read_exact(buf.as_mut_slice()).unwrap();
            println!("Saving PSP HDCP to {}", output);
            std::fs::write(output, buf).unwrap();
        }
        "psp_dtm" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            println!("{:#x?}", ta_fw);
            if ta_fw.dtm.size == 0 {
                eprintln!("DTM size 0!");
                return;
            }
            c.set_position(ucode_off as u64 + ta_fw.dtm.off as u64);
            let mut buf = Vec::new();
            buf.resize(ta_fw.dtm.size as usize, 0);
            c.read_exact(buf.as_mut_slice()).unwrap();
            println!("Saving PSP DTM to {}", output);
            std::fs::write(output, buf).unwrap();
        }
        "psp_secure_display" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            println!("{:#x?}", ta_fw);
            if ta_fw.secure_display.size == 0 {
                eprintln!("Secure Display size 0!");
                return;
            }
            c.set_position(ucode_off as u64 + ta_fw.secure_display.off as u64);
            let mut buf = Vec::new();
            buf.resize(ta_fw.secure_display.size as usize, 0);
            c.read_exact(buf.as_mut_slice()).unwrap();
            println!("Saving PSP Secure Display to {}", output);
            std::fs::write(output, buf).unwrap();
        }
        "psp_asd" => {
            c.set_position(ucode_off as u64);
            let mut buf = Vec::new();
            buf.resize(ucode_size as usize, 0);
            c.read_exact(buf.as_mut_slice()).unwrap();
            println!("Saving PSP ASD to {}", output);
            std::fs::write(output, buf).unwrap();
        }
        v => eprintln!("Unknown firmware type '{}'", v),
    }
}
