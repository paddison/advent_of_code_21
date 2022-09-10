// indicates where the body of a package starts
const HEADER_LEN: usize = 6;

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

impl From<u8> for TypeID {
    fn from(val: u8) -> Self {
        match val {
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
    let input = include_str!("../data/day_16.txt");
    let transmission = into_binary(input);
    let mut version_sum = 0;
    parse_package(&transmission, 0, &mut version_sum);

    version_sum
}

pub fn get_solution_2() -> u64 {
    let input = include_str!("../data/day_16.txt");
    let transmission = into_binary(input);

    parse_package(&transmission, 0, &mut 0).0
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

fn parse_package(pkg: &str, offset: usize, version_sum: &mut u32) -> (u64, usize) {
    let (version, p_type) = parse_header(&pkg[offset..]);
    *version_sum += version;

    match p_type {
        PackageType::Operator(id) => parse_operator(&pkg[offset..], id, version_sum),
        PackageType::Literal => parse_literal(&pkg[offset..]), 
    }
}

fn parse_header(pkg: &str) -> (u32, PackageType) {
    let version = u32::from_str_radix(&pkg[0..3], 2).unwrap();
    match u8::from_str_radix(&pkg[3..6], 2) {
        Ok(4) => (version, PackageType::Literal),
        Ok(id) => (version, PackageType::Operator(id.into())),
        Err(_) => panic!("Got invalid header."),
    }
}

fn parse_literal(pkg: &str) -> (u64, usize) {
    let mut cursor = HEADER_LEN;
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

// cursor points to first bit after header
fn parse_operator(pkg: &str, id: TypeID, version_sum: &mut u32) -> (u64, usize) {
    let mut values = vec![];

    // switch up length types, so we can calculate the subpackages left later
    let (mut offset, len_type) = if &pkg[HEADER_LEN..HEADER_LEN + 1] == "0" { (16, 1) } else { (12, 0) };
    
    offset += HEADER_LEN;
    
    let mut subpackages = u16::from_str_radix(&pkg[HEADER_LEN + 1..offset], 2).unwrap() as usize;

    while subpackages > 0 {
        let (value, parsed_bits) = parse_package(pkg, offset, version_sum);

        values.push(value);
        subpackages -= parsed_bits.pow(len_type); //basically this means: if len_type == 1 { parsed_bits } else { 1 };
        offset += parsed_bits;
    }
    
    (compute_operator(values, id), offset)
}

fn compute_operator(values: Vec<u64>, id: TypeID) -> u64 {
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
    use super::{into_binary, parse_literal, parse_operator, parse_package};
    
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
        let (actual_val, actual_cur) = parse_literal("110100101111111000101000");

        assert_eq!(actual_val, 2021);
        assert_eq!(actual_cur, 21);
    }

    #[test]
    fn test_parse_operator() {
        let pkg1 = "00111000000000000110111101000101001010010001001000000000";
        let (_, actual_cursor1) = parse_operator(pkg1, 6.into(), &mut 0);

        assert_eq!(actual_cursor1, 49);

        let pkg2 = "11101110000000001101010000001100100000100011000001100000";
        let (_, actual_cursor2) = parse_operator(pkg2, 3.into(), &mut 0);

        assert_eq!(actual_cursor2, 51);
    }

    #[test]
    fn test_compute_operator() {
        let pkg = into_binary("C200B40A82");
        let actual_value = parse_package(&pkg, 0, &mut 0);
        assert_eq!(actual_value.0, 3);
        

        let pkg = into_binary("04005AC33890");
        let actual_value = parse_package(&pkg, 0, &mut 0);
        
        assert_eq!(actual_value.0, 54);

        let pkg = into_binary("880086C3E88112");
        let actual_value = parse_package(&pkg, 0, &mut 0);
        
        assert_eq!(actual_value.0, 7);

        let pkg = into_binary("CE00C43D881120");
        let actual_value = parse_package(&pkg, 0, &mut 0);
        
        assert_eq!(actual_value.0, 9);

        let pkg = into_binary("D8005AC2A8F0");
        let actual_value = parse_package(&pkg, 0, &mut 0);
        
        assert_eq!(actual_value.0, 1);

        let pkg = into_binary("F600BC2D8F");
        let actual_value = parse_package(&pkg, 0, &mut 0);
        
        assert_eq!(actual_value.0, 0);

        let pkg = into_binary("9C005AC2F8F0");
        let actual_value = parse_package(&pkg, 0, &mut 0);
        
        assert_eq!(actual_value.0, 0);

        let pkg = into_binary("9C0141080250320F1802104A08");
        let actual_value = parse_package(&pkg, 0, &mut 0);
        
        assert_eq!(actual_value.0, 1);
    }
}