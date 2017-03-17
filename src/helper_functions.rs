pub fn display_reg32(reg32: u32) -> String {
    let sign_symbol = if reg32 & (1u32 << 30) == 0 { "+" } else { "-" };
    let bytes = (1..5).map(|i| {
        (reg32 >> 6*(5 - i)) % (1u32 << 5)
    }).collect::<Vec<_>>();
    format!("[{}|{}|{}|{}|{}|{}]", sign_symbol, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4])
}

mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let reg1 = (50u32 << 6*4) + (40u32 << 6*3) + (30u32 << 6*2) + (20u32 << 6*1) + (10u32);

        assert_eq!(display_reg32(reg1), String::from("[+|50|40|30|20|10]"));
    }
}
