pub trait PotatoPrimitive {}

pub enum PotatoType {
    // Ints
    Int(i32),
    BigInt(i64),
    BiggerInt(i128),

    // Unsigned ints
    UInt(u32),
    BigUInt(u64),
    BiggerUInt(u128),

    // Floats
    Float(f32),
    Double(f64),

    // Bool
    Bool(bool),

    // String
    String(String),
}

impl PotatoPrimitive for PotatoType {}
