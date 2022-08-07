use crate::parse_lines;

const HEADER_LEN: usize = 6;
// static mut VERSION_N: u32 = 0;
const SOLUTION_1: u32 = 938;

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

    while let (_, Some(next_pos)) = parse_package(&transmission[cursor..]) {
        cursor += next_pos;
    }

    SOLUTION_1
}

pub fn get_solution_2() -> u64 {
    let input = parse_lines("data/day_16.txt");
    let transmission = into_binary(&input[0]);
    let mut cursor = 0;
    
    loop {
        match parse_package(&transmission[cursor..]) {
            (_value, Some(next_pos)) => cursor += next_pos, // this case doesn't seem to happen
            (value, None) => break value,
        }
    }    
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

fn parse_package(pkg: &str) -> (u64, Option<usize>) {
    let (value, cursor) = match parse_header(pkg) {
        (_version, PackageType::Operator(id)) => parse_operator(&pkg, HEADER_LEN, id),
        (_version, PackageType::Literal) => parse_literal(&pkg, HEADER_LEN), 
    };
    
    // find start of next package
    (value, determine_next(pkg, cursor))
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

fn parse_subpackages(pkg: &str, offset: usize) -> (u64, usize) {
    let (value, cur_cursor) = match parse_header(&pkg[offset..]) {
        (_version, PackageType::Operator(id)) => parse_operator(&pkg[offset..], HEADER_LEN, id),
        (_version, PackageType::Literal) => parse_literal(&pkg[offset..], HEADER_LEN),
    };

    (value, cur_cursor)
}

// cursor points to first bit after header
fn parse_operator(pkg: &str, cursor: usize, id: TypeID) -> (u64, usize) {
    // determine type of L field
    let mut offset = cursor;
    let mut values = vec![];
    
    if &pkg[cursor..cursor + 1] == "0" {
        offset += 16;
        let mut n_bits = u16::from_str_radix(&pkg[cursor + 1..offset], 2).unwrap() as usize;
        
        // add up length of parsed packages
        while n_bits > 0 {
            // parse subpackages
            let (value, parsed_bits) = parse_subpackages(&pkg, offset);
            values.push(value);
            n_bits -= parsed_bits;
            offset += parsed_bits;
        }
    } else {
        offset += 12;
        let n_packages = u16::from_str_radix(&pkg[cursor + 1..offset], 2).unwrap();
        
        for _ in 0..n_packages {
            let (value, parsed_bits) = parse_subpackages(&pkg, offset);
            values.push(value);
            offset += parsed_bits;
        }
    }
    
    let result = compute_package(values, id);

    (result, offset)
}

fn compute_package(values: Vec<u64>, id: TypeID) -> u64 {
    match id {
        TypeID::Sum => values.into_iter().sum(),
        TypeID::Prod => values.into_iter().product(),
        TypeID::Min => values.into_iter().min().unwrap(),
        TypeID::Max => values.into_iter().max().unwrap(),
        TypeID::GT => if values[0] > values[1] { 1 } else { 0 },
        TypeID::LT => if values[0] < values[1] { 1 } else { 0 },
        TypeID::Eq => if values[0] == values[1] { 1 } else { 0 },
    }
}

#[cfg(test)]
mod tests {
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
    fn test_compute_operator() {
        let pkg = into_binary("C200B40A82");
        let (actual_value, offset) = parse_package(&pkg);
        
        assert!(offset.is_none());
        assert_eq!(actual_value, 3);

        let pkg = into_binary("04005AC33890");
        let (actual_value, offset) = parse_package(&pkg);
        
        assert!(offset.is_none());
        assert_eq!(actual_value, 54);

        let pkg = into_binary("880086C3E88112");
        let (actual_value, offset) = parse_package(&pkg);
        
        assert!(offset.is_none());
        assert_eq!(actual_value, 7);

        let pkg = into_binary("CE00C43D881120");
        let (actual_value, offset) = parse_package(&pkg);
        
        assert!(offset.is_none());
        assert_eq!(actual_value, 9);

        let pkg = into_binary("D8005AC2A8F0");
        let (actual_value, offset) = parse_package(&pkg);
        
        assert!(offset.is_none());
        assert_eq!(actual_value, 1);

        let pkg = into_binary("F600BC2D8F");
        let (actual_value, offset) = parse_package(&pkg);
        
        assert!(offset.is_none());
        assert_eq!(actual_value, 0);

        let pkg = into_binary("9C005AC2F8F0");
        let (actual_value, offset) = parse_package(&pkg);
        
        assert!(offset.is_none());
        assert_eq!(actual_value, 0);

        let pkg = into_binary("9C0141080250320F1802104A08");
        let (actual_value, offset) = parse_package(&pkg);
        
        assert!(offset.is_none());
        assert_eq!(actual_value, 1);
    }

    // not working anymore
    // #[test]
    // fn test_add_versions() {
    //     let pkg1 = into_binary("8A004A801A8002F478");
    //     let _ = parse_package(&pkg1);

    //     unsafe { 
    //         assert_eq!(VERSION_N, 16);
    //         VERSION_N = 0; 
    //     }

    //     let pkg2 = into_binary("620080001611562C8802118E34");
    //     let _ = parse_package(&pkg2);

    //     unsafe { 
    //         assert_eq!(VERSION_N, 12);
    //         VERSION_N = 0; 
    //     }

    //     let pkg3 = into_binary("C0015000016115A2E0802F182340");
    //     let _ = parse_package(&pkg3);

    //     unsafe { 
    //         assert_eq!(VERSION_N, 23);
    //         VERSION_N = 0; 
    //     }

    //     let pkg4 = into_binary("A0016C880162017C3686B18A3D4780");
    //     let _ = parse_package(&pkg4);

    //     unsafe { 
    //         assert_eq!(VERSION_N, 31);
    //         VERSION_N = 0; 
    //     }
    // }
}