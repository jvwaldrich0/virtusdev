use crc::{Crc, CRC_16_MODBUS, CRC_16_ARC, CRC_16_IBM_SDLC, CRC_16_KERMIT, CRC_16_XMODEM, CRC_16_USB};

fn main() {
    let data = vec![0x80, 0x01, 0x11];
    let expected = 0x8265;

    println!("Testing CRC algorithms for data {:02X?} expecting 0x{:04X}", data, expected);

    let algos = [
        ("MODBUS (8005, FFFF, ref)", Crc::<u16>::new(&CRC_16_MODBUS)),
        ("ARC (8005, 0000, ref)", Crc::<u16>::new(&CRC_16_ARC)),
        ("IBM_SDLC (1021, FFFF, ref)", Crc::<u16>::new(&CRC_16_IBM_SDLC)),
        ("KERMIT (1021, 0000, ref)", Crc::<u16>::new(&CRC_16_KERMIT)),
        ("XMODEM (1021, 0000, noref)", Crc::<u16>::new(&CRC_16_XMODEM)),
        ("USB (8005, FFFF, ref, xor FFFF)", Crc::<u16>::new(&CRC_16_USB)),
    ];

    for (name, crc) in algos.iter() {
        let result = crc.checksum(&data);
        println!("{}: 0x{:04X} {}", name, result, if result == expected { "MATCH!" } else { "" });
    }
    
    // Test manual custom params if needed
    // ITL spec says: poly 0x8005, init 0xFFFF. Usually implies not reflected if not specified, or MODBUS if reflected.
    const CUSTOM_ALGO: crc::Algorithm<u16> = crc::Algorithm {
        width: 16,
        poly: 0x8005,
        init: 0xFFFF,
        refin: false,
        refout: false,
        xorout: 0x0000,
        check: 0,
        residue: 0
    };
    let crc_custom = Crc::<u16>::new(&CUSTOM_ALGO);
    let res = crc_custom.checksum(&data);
    println!("Custom (8005, FFFF, noref): 0x{:04X} {}", res, if res == expected { "MATCH!" } else { "" });
}
