pub fn display_reg32(reg32: u32) -> String {
    let sign_symbol = if reg32 & (1u32 << 30) == 0 { "+" } else { "-" };
    let bytes = (1..6).map(|i| {
        (reg32 >> 6*(5 - i)) % (1u32 << 6)
    }).collect::<Vec<_>>();
    format!("[{}|{}|{}|{}|{}|{}]", sign_symbol, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4])
}

pub fn display_reg16(reg16: u16) -> String {
    let sign_symbol = if reg16 & (1u16 << 12) == 0 { "+" } else { "-" };
    let bytes = (1..3).map(|i| {
        (reg16 >> 6*(2 - i)) % (1u16 << 6)
    }).collect::<Vec<_>>();
    format!("[{}|{}|{}]", sign_symbol, bytes[0], bytes[1])
}

mod tests {
    use super::*;

    #[test]
    fn test_display_reg32() {
        let reg1 = (50u32 << 6*4) + (40u32 << 6*3) + (30u32 << 6*2) + (20u32 << 6*1) + (10u32);
        assert_eq!(display_reg32(reg1), String::from("[+|50|40|30|20|10]"));

        let reg2 = (50u32 << 6*4) + (40u32 << 6*3) + (30u32 << 6*2) + (20u32 << 6*1) + (10u32) + (1u32 << 6*5);
        assert_eq!(display_reg32(reg2), String::from("[-|50|40|30|20|10]"));
    }

    #[test]
    fn test_display_reg16() {
        let reg1 = (50u16 << 6) + (40u16);
        assert_eq!(display_reg16(reg1), String::from("[+|50|40]"));

        let reg2 = (50u16 << 6) + (40u16) + (1u16 << 6*2);
        assert_eq!(display_reg16(reg2), String::from("[-|50|40]"));
    }
}
