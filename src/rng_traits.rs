use std::fmt::{Debug, Display};

pub trait Randomizable: Display + Debug {
    fn random() -> Self;
}

// BYTE
impl Randomizable for u8 {
    fn random() -> Self {
        let mut number = [0u8];
        getrandom::fill(&mut number).expect("getting random numbers should be possible");
        number[0]
    }
    
}
impl Randomizable for i8 {
    fn random() -> Self {
        let mut buf = [0u8];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i8::from_ne_bytes(buf)
    }
}
// SHORT
impl Randomizable for u16 {
    fn random() -> Self {
        let mut buf = [0u8; 2];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        u16::from_ne_bytes(buf)
    }
}
impl Randomizable for i16 {
    fn random() -> Self {
        let mut buf = [0u8; 2];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i16::from_ne_bytes(buf)
    }
}
// INT
impl Randomizable for u32 {
    fn random() -> Self {
        let mut buf = [0u8; 4];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        u32::from_ne_bytes(buf)
    }
}
impl Randomizable for i32 {
    fn random() -> Self {
        let mut buf = [0u8; 4];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i32::from_ne_bytes(buf)
    }
}
// LONG
impl Randomizable for u64 {
    fn random() -> Self {
        let mut buf = [0u8; 8];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        u64::from_ne_bytes(buf)
    }
}
impl Randomizable for i64 {
    fn random() -> Self {
        let mut buf = [0u8; 8];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i64::from_ne_bytes(buf)
    }
}
// COMICALLY LARGE
impl Randomizable for u128 {
    fn random() -> Self {
        let mut buf = [0u8; 16];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        u128::from_ne_bytes(buf)
    }
}
impl Randomizable for i128 {
    fn random() -> Self {
        let mut buf = [0u8; 16];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i128::from_ne_bytes(buf)
    }
}

// USIZE
impl Randomizable for usize {
    fn random() -> Self {
        let mut buf = [0u8; size_of::<usize>()];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        usize::from_ne_bytes(buf)
    }
}

// DECIMALS
impl Randomizable for f32 {
    fn random() -> Self {
        let mut buf = [0u8; 4];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        f32::from_ne_bytes(buf)
    }
}
impl Randomizable for f64 {
    fn random() -> Self {
        let mut buf = [0u8; 8];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        f64::from_ne_bytes(buf)
    }
}