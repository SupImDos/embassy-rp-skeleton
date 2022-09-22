use rgb::RGB8;

const RANGE: core::ops::RangeInclusive<u8> = u8::MIN..=u8::MAX;

/// Generates an infinite iterator of rainbow RGB tuples.
pub fn rainbow() -> impl Iterator<Item = RGB8> + Clone {
    // (FF, 00, 00) -> (FF, FF, 00)
    // (FF, FF, 00) -> (00, FF, 00)
    // (00, FF, 00) -> (00, FF, FF)
    // (00, FF, FF) -> (00, 00, FF)
    // (00, 00, FF) -> (FF, 00, FF)
    // (FF, 00, FF) -> (FF, 00, 00)
    core::iter::empty()
        .chain(RANGE.map(|n| RGB8::new(u8::MAX, n, u8::MIN)))
        .chain(RANGE.rev().map(|n| RGB8::new(n, u8::MAX, u8::MIN)))
        .chain(RANGE.map(|n| RGB8::new(u8::MIN, u8::MAX, n)))
        .chain(RANGE.rev().map(|n| RGB8::new(u8::MIN, n, u8::MAX)))
        .chain(RANGE.map(|n| RGB8::new(n, u8::MIN, u8::MAX)))
        .chain(RANGE.rev().map(|n| RGB8::new(u8::MAX, u8::MIN, n)))
        .skip(1) // Skip to avoid repeat on wrap-around
        .cycle()
}

pub fn n(c: &RGB8) -> RGB8 {
    match (c.r, c.g, c.b) {
        (0xFF, 0x00, 0x00) => RGB8::new(0xFF, 0x01, 0x00),
        (0xFF, 0xFF, 0x00) => RGB8::new(0xFE, 0xFF, 0x00),
        (0x00, 0xFF, 0x00) => RGB8::new(0x00, 0xFF, 0x01),
        (0x00, 0xFF, 0xFF) => RGB8::new(0x00, 0xFE, 0xFF),
        (0x00, 0x00, 0xFF) => RGB8::new(0x01, 0x00, 0xFF),
        (0xFF, 0x00, 0xFF) => RGB8::new(0xFF, 0x00, 0xFE),
        (0xFF, g, 0x00) => RGB8::new(0xFF, g + 1, 0x00),
        (r, 0xFF, 0x00) => RGB8::new(r - 1, 0xFF, 0x00),
        (0x00, 0xFF, b) => RGB8::new(0x00, 0xFF, b + 1),
        (0x00, g, 0xFF) => RGB8::new(0x00, g - 1, 0xFF),
        (r, 0x00, 0xFF) => RGB8::new(r + 1, 0x00, 0xFF),
        (0xFF, 0x00, b) => RGB8::new(0xFF, 0x00, b - 1),
        _ => unreachable!(),
    }
}
