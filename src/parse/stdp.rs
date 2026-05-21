use super::Parser;
use std::num::NonZeroU32;

/// Беззнаковые числа
#[derive(Debug)]
pub struct U32;
impl Parser for U32 {
    type Dest = NonZeroU32;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, is_hex) = input
            .strip_prefix("0x")
            .map_or((input, false), |rem| (rem, true));
        let end_idx = remaining
            .char_indices()
            .find_map(|(idx, c)| match (is_hex, c) {
                (true, 'a'..='f' | '0'..='9' | 'A'..='F') => None,
                (false, '0'..='9') => None,
                _ => Some(idx),
            })
            .unwrap_or(remaining.len());
        let raw = u32::from_str_radix(&remaining[..end_idx], if is_hex { 16 } else { 10 })
            .map_err(|_| ())?;
        NonZeroU32::new(raw)
            .ok_or(())
            .map(|nz| (&remaining[end_idx..], nz))
    }
}
/// Знаковые числа
#[derive(Debug)]
pub struct I32;
impl Parser for I32 {
    type Dest = i32;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, i32), ()> {
        let mut chars = input.char_indices();
        let start = match chars.next() {
            Some((_, '+' | '-')) => 1,
            Some(_) => 0,
            None => return Err(()),
        };
        let end_idx = input
            .char_indices()
            .skip(start)
            .find_map(|(idx, c)| if !c.is_ascii_digit() { Some(idx) } else { None })
            .unwrap_or(input.len());
        if end_idx == 0 || (start == 1 && end_idx == 1) {
            return Err(()); // знак без цифр
        }
        let value: i32 = input[..end_idx].parse().map_err(|_| ())?;
        Ok((&input[end_idx..], value))
    }
}
/// Шестнадцатеричные байты (пригодится при парсинге блобов)
#[derive(Debug, Clone)]
pub struct Byte;
impl Parser for Byte {
    type Dest = u8;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (to_parse, remaining) = input.split_at_checked(2).ok_or(())?;
        if !to_parse.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(());
        }
        let value = u8::from_str_radix(to_parse, 16).map_err(|_| ())?;
        Ok((remaining, value))
    }
}
