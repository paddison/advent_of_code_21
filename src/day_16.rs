use crate::parse_lines;

const HEADER_LEN: usize = 6;
static mut VERSION_N: u32 = 0;

enum PackageType {
    Operator(TypeID),
    Literal,
}

enum TypeID {
    Sum,
    Prod,
    Min,
    Max,
    GT,
    LT,
    Eq,
}

impl Into<TypeID> for u8 {
    fn into(self) -> TypeID {
        match self {
            0 => TypeID::Sum,
            1 => TypeID::Prod,
            2 => TypeID::Min,
            3 => TypeID::Max,
            5 => TypeID::GT,
            6 => TypeID::LT,
            7 => TypeID::Eq,
            _ => panic!("Got Invalid Type Id in transmission"),
        }
    }
}

pub fn get_solution_1() -> u32 {
    let input = parse_lines("data/day_16.txt");
    let transmission = into_binary(&input[0]);
    let mut cursor = 0;

    while let Some(next_pos) = parse_package(&transmission[cursor..]) {
        cursor += next_pos;
    }

    unsafe { VERSION_N }
}

fn into_binary(transmission: &str) -> String {
    let mut bin_transmission = String::new();
    for ch in transmission.chars() {
        bin_transmission += match ch {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("got non hex char"),
        };
    }

    bin_transmission
}

fn parse_package(pkg: &str) -> Option<usize> {
    let (_, mut cursor) = match parse_header(pkg) {
        (version, PackageType::Operator(id)) => { 
            unsafe { VERSION_N += version; }  
            parse_operator(&pkg, HEADER_LEN, id)
        },
        (version, PackageType::Literal) => { 
            unsafe { VERSION_N += version; }
            parse_literal(&pkg, HEADER_LEN) }, 
    };
    
    // find start of next package
    determine_next(pkg, cursor) 
}

fn parse_header(pkg: &str) -> (u32, PackageType) {
    let version = u32::from_str_radix(&pkg[0..3], 2).unwrap();
    match u8::from_str_radix(&pkg[3..6], 2) {
        Ok(4) => (version, PackageType::Literal),
        Ok(id) => (version, PackageType::Operator(id.into())),
        Err(_) => panic!(),
    }
}

fn determine_next(pkg: &str, mut cursor: usize) -> Option<usize> {
    // find start of next package
    let offset = 4 - (cursor % 4);
    cursor += offset; // adjust cursor to point to 4 bit boundary
    while let Some(val) = pkg.get(cursor..cursor + 3) {
        if val.contains("1") {
            return Some(cursor)
        }
        cursor += 4;
    }

    None
}

fn parse_literal(pkg: &str, mut cursor: usize) -> (u64, usize) {

    let mut n = String::new();

    loop {
        n += &pkg[cursor + 1..cursor + 5];
        if &pkg[cursor..cursor + 1] == "0" {
            break;
        }
        cursor += 5;
    };

    (u64::from_str_radix(&n, 2).unwrap(), cursor + 5)
}

fn parse_subpackages(pkg: &str, len_offset: usize, cursor: usize, parsed_bits: usize) -> usize {
    let (_, cur_cursor) = match parse_header(&pkg[cursor + len_offset + parsed_bits..]) {
        (version, PackageType::Operator(id)) => {
            unsafe { VERSION_N += version; }  
            parse_operator(&pkg[cursor + len_offset + parsed_bits..], HEADER_LEN, id) 
        },
        (version, PackageType::Literal) => {
            unsafe { VERSION_N += version; }     
            parse_literal(&pkg[cursor + len_offset + parsed_bits..], HEADER_LEN)
        },
    };

    cur_cursor
}

fn parse_operator(pkg: &str, cursor: usize, id: TypeID) -> (u64, usize) {
    // determine type of L field
    if &pkg[cursor..cursor + 1] == "0" {
        let len_offset = 16;
        let n_bits = u16::from_str_radix(&pkg[cursor + 1..cursor + len_offset], 2).unwrap() as usize;
        // add up length of parsed packages
        let mut parsed_bits = 0;
        while parsed_bits != n_bits {
            // parse subpackages
            parsed_bits += parse_subpackages(&pkg, len_offset, cursor, parsed_bits);
            // let (_, cur_cursor) = match parse_header(&pkg[cursor + 16 + parsed_bits..]) {
            //     (version, PackageType::Operator(id)) => {
            //         unsafe { VERSION_N += version; }  
            //         parse_operator(&pkg[cursor + 16 + parsed_bits..], HEADER_LEN, id) 
            //     },
            //     (version, PackageType::Literal) => {
            //         unsafe { VERSION_N += version; }     
            //         parse_literal(&pkg[cursor + 16 + parsed_bits..], HEADER_LEN)
            //     },
            // };

            // parsed_bits += cur_cursor;
        }
        (0, cursor + parsed_bits + 16)
    } else {
        let len_offset = 12;
        let n_packages = u16::from_str_radix(&pkg[cursor + 1..cursor + len_offset], 2).unwrap();
        let mut parsed_bits = 0;
        for _ in 0..n_packages {
            parsed_bits += parse_subpackages(&pkg, len_offset, cursor, parsed_bits);
            // let (_, cur_cursor) = match parse_header(&pkg[cursor + 12 + parsed_bits..]) {
            //     (version, PackageType::Operator(id)) => { 
            //         unsafe { VERSION_N += version; }  
            //         parse_operator(&pkg[cursor + 12 + parsed_bits..], HEADER_LEN, id)
            //     },
            //     (version, PackageType::Literal) => {
            //         unsafe { VERSION_N += version; }  
            //         parse_literal(&pkg[cursor + 12 + parsed_bits..], HEADER_LEN)
            //     },
            // };

            // parsed_bits += cur_cursor;
        }
        (0, cursor + parsed_bits + 12)
    }
}

#[cfg(test)]
mod tests {
    use crate::day_16::VERSION_N;

    use super::{into_binary, parse_literal, HEADER_LEN, determine_next, parse_operator, parse_package};
    
    #[test]
    fn test_into_binary() {
        let s1 = "AC";
        let expected1 = "10101100".to_string();
        assert_eq!(into_binary(s1), expected1);

        let s2 = "0123";
        let expected2 = "0000000100100011".to_string();
        assert_eq!(into_binary(s2), expected2);

        let s3 = "ABCD04215467899FFE";
        let expected3 = "101010111100110100000100001000010101010001100111100010011001111111111110".to_string();
        assert_eq!(into_binary(s3), expected3);
    }

    #[test]
    fn test_parse_literal() {
        let (actual_val, actual_cur) = parse_literal("110100101111111000101000", HEADER_LEN);

        assert_eq!(actual_val, 2021);
        assert_eq!(actual_cur, 21);
    }

    #[test]
    fn test_determine_next() {
        let pkg1 = "110100101111111000101000001";
        let (_, cursor1) = parse_literal(pkg1, HEADER_LEN);
        let actual_next = determine_next(pkg1, cursor1);
        
        assert_eq!(actual_next, Some(24));

        let pkg2 = "1101001011111110001010000000100";
        let (_, cursor2) = parse_literal(pkg2, HEADER_LEN);
        let actual_next = determine_next(pkg2, cursor2);
        
        assert_eq!(actual_next, Some(28));
        
        let pkg3 = "11010010111111100010100000000000010";
        let (_, cursor3) = parse_literal(pkg3, HEADER_LEN);
        let actual_next = determine_next(pkg3, cursor3);

        assert_eq!(actual_next, Some(32));
    }

    #[test]
    fn test_parse_operator() {
        let pkg1 = "00111000000000000110111101000101001010010001001000000000";
        let (_, actual_cursor1) = parse_operator(pkg1, HEADER_LEN, 6.into());

        assert_eq!(actual_cursor1, 49);

        let pkg2 = "11101110000000001101010000001100100000100011000001100000";
        let (_, actual_cursor2) = parse_operator(pkg2, HEADER_LEN, 3.into());

        assert_eq!(actual_cursor2, 51);
    }

    #[test]
    fn test_add_versions() {
        let pkg1 = into_binary("8A004A801A8002F478");
        let _ = parse_package(&pkg1);

        unsafe { 
            assert_eq!(VERSION_N, 16);
            VERSION_N = 0; 
        }

        let pkg2 = into_binary("620080001611562C8802118E34");
        let _ = parse_package(&pkg2);

        unsafe { 
            assert_eq!(VERSION_N, 12);
            VERSION_N = 0; 
        }

        let pkg3 = into_binary("C0015000016115A2E0802F182340");
        let _ = parse_package(&pkg3);

        unsafe { 
            assert_eq!(VERSION_N, 23);
            VERSION_N = 0; 
        }

        let pkg4 = into_binary("A0016C880162017C3686B18A3D4780");
        let _ = parse_package(&pkg4);

        unsafe { 
            assert_eq!(VERSION_N, 31);
            VERSION_N = 0; 
        }
    }
}