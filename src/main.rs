use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
struct FwLegacyBinDesc {
    _ver: u32,
    off: u32,
    size: u32,
}

impl FwLegacyBinDesc {
    pub fn from_reader<T: std::io::Read>(r: &mut T) -> Self {
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
    pub fn from_reader<T: std::io::Read>(r: &mut T) -> Self {
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
    let ty = std::env::args().nth(1).unwrap();
    let input = std::env::args().nth(2).unwrap();
    let output = std::env::args().nth(3).unwrap();

    let mut c = std::io::Cursor::new(std::fs::read(&input).unwrap());
    c.set_position(20);
    let ucode_size = c.read_u32::<LittleEndian>().unwrap() as usize;
    let ucode_off = c.read_u32::<LittleEndian>().unwrap() as u64;
    c.seek(SeekFrom::Current(4)).unwrap();

    match ty.as_str() {
        "xgmi" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            assert_ne!(ta_fw.xgmi.off, 0);
            assert_ne!(ta_fw.xgmi.size, 0);
            c.set_position(ucode_off + ta_fw.xgmi.off as u64);
            let mut buf = vec![0; ta_fw.xgmi.size as usize];
            c.read_exact(buf.as_mut_slice()).unwrap();
            std::fs::write(output, buf).unwrap();
        }
        "ras" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            assert_ne!(ta_fw.ras.off, 0);
            assert_ne!(ta_fw.ras.size, 0);
            c.set_position(ucode_off + ta_fw.ras.off as u64);
            let mut buf = vec![0; ta_fw.ras.size as usize];
            c.read_exact(buf.as_mut_slice()).unwrap();
            std::fs::write(output, buf).unwrap();
        }
        "hdcp" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            assert_ne!(ta_fw.hdcp.off, 0);
            assert_ne!(ta_fw.hdcp.size, 0);
            c.set_position(ucode_off + ta_fw.hdcp.off as u64);
            let mut buf = vec![0; ta_fw.hdcp.size as usize];
            c.read_exact(buf.as_mut_slice()).unwrap();
            std::fs::write(output, buf).unwrap();
        }
        "dtm" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            assert_ne!(ta_fw.dtm.off, 0);
            assert_ne!(ta_fw.dtm.size, 0);
            c.set_position(ucode_off + ta_fw.dtm.off as u64);
            let mut buf = vec![0; ta_fw.dtm.size as usize];
            c.read_exact(buf.as_mut_slice()).unwrap();
            std::fs::write(output, buf).unwrap();
        }
        "secure_display" => {
            let ta_fw = PspTaFw::from_reader(&mut c);
            assert_ne!(ta_fw.secure_display.off, 0);
            assert_ne!(ta_fw.secure_display.size, 0);
            c.set_position(ucode_off + ta_fw.secure_display.off as u64);
            let mut buf = vec![0; ta_fw.secure_display.size as usize];
            c.read_exact(buf.as_mut_slice()).unwrap();
            std::fs::write(output, buf).unwrap();
        }
        "asd" => {
            c.set_position(ucode_off);
            let mut buf = vec![0; ucode_size];
            c.read_exact(buf.as_mut_slice()).unwrap();
            std::fs::write(output, buf).unwrap();
        }
        "gfx" => {
            c.seek(SeekFrom::Current(4)).unwrap();
            let jt_off = c.read_u32::<LittleEndian>().unwrap() as u64;
            let jt_size = c.read_u32::<LittleEndian>().unwrap() as usize * 4;
            let mec = input.ends_with("_mec.bin") || input.ends_with("_mec2.bin");
            c.set_position(ucode_off);
            let mut buf = vec![
                0;
                if mec {
                    ucode_size - jt_size
                } else {
                    ucode_size
                }
            ];
            c.read_exact(buf.as_mut_slice()).unwrap();
            std::fs::write(&output, buf).unwrap();
            println!("jt_off: {:#X}", jt_off);
            if jt_off != 0 && jt_size != 0 && mec {
                c.set_position(ucode_off + (ucode_size - jt_size) as u64);
                let mut buf = vec![0; jt_size];
                c.read_exact(buf.as_mut_slice()).unwrap();
                std::fs::write(output + ".jt", buf).unwrap();
            }
        }
        v => eprintln!("Unknown firmware type '{}'", v),
    }
}
