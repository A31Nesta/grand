use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug, Clone, PartialEq)]
pub struct RangeError<T: Display + Debug> {
    should_be_smaller: T,
    should_be_bigger: T
}
impl<T: Display + Debug> RangeError<T> {
    pub fn new(should_be_smaller: T, should_be_bigger: T) -> Self {
        RangeError {
            should_be_smaller: should_be_smaller,
            should_be_bigger: should_be_bigger
        }
    }
}
impl<T: Display + Debug> Display for RangeError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid range, x ({}) should be smaller than y ({})", self.should_be_smaller, self.should_be_bigger)
    }
}
impl<T: Display + Debug> Error for RangeError<T> {}

type RangeResult<T> = Result<Box<T>, RangeError<Box<T>>>;

pub trait Randomizable: Display + Debug {
    fn random() -> Self;
    fn random_range(x: Self, y: Self) -> RangeResult<Self>;
}

// BYTE
impl Randomizable for u8 {
    fn random() -> Self {
        let mut number = [0u8];
        getrandom::fill(&mut number).expect("getting random numbers should be possible");
        number[0]
    }
    
    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        res = res % (corrected_diff) + x;

        Ok(Box::new(res))
    }
    
}
impl Randomizable for i8 {
    fn random() -> Self {
        let mut buf = [0u8];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i8::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        let offset = if res < 0 { y } else { x };
        res = res % (corrected_diff) + offset;

        Ok(Box::new(res))
    }
}
// SHORT
impl Randomizable for u16 {
    fn random() -> Self {
        let mut buf = [0u8; 2];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        u16::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        res = res % (corrected_diff) + x;

        Ok(Box::new(res))
    }
}
impl Randomizable for i16 {
    fn random() -> Self {
        let mut buf = [0u8; 2];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i16::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        let offset = if res < 0 { y } else { x };
        res = res % (corrected_diff) + offset;

        Ok(Box::new(res))
    }
}
// INT
impl Randomizable for u32 {
    fn random() -> Self {
        let mut buf = [0u8; 4];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        u32::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        res = res % (corrected_diff) + x;

        Ok(Box::new(res))
    }
}
impl Randomizable for i32 {
    fn random() -> Self {
        let mut buf = [0u8; 4];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i32::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        let offset = if res < 0 { y } else { x };
        res = res % (corrected_diff) + offset;

        Ok(Box::new(res))
    }
}
// LONG
impl Randomizable for u64 {
    fn random() -> Self {
        let mut buf = [0u8; 8];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        u64::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        res = res % (corrected_diff) + x;

        Ok(Box::new(res))
    }
}
impl Randomizable for i64 {
    fn random() -> Self {
        let mut buf = [0u8; 8];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i64::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        let offset = if res < 0 { y } else { x };
        res = res % (corrected_diff) + offset;

        Ok(Box::new(res))
    }
}
// COMICALLY LARGE
impl Randomizable for u128 {
    fn random() -> Self {
        let mut buf = [0u8; 16];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        u128::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        res = res % (corrected_diff) + x;

        Ok(Box::new(res))
    }
}
impl Randomizable for i128 {
    fn random() -> Self {
        let mut buf = [0u8; 16];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        i128::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let corrected_diff = if diff == Self::MAX { diff } else { diff+1 };
        let mut res = Self::random();
        let offset = if res < 0 { y } else { x };
        res = res % (corrected_diff) + offset;

        Ok(Box::new(res))
    }
}

// DECIMALS
impl Randomizable for f32 {
    fn random() -> Self {
        let mut buf = [0u8; 4];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        f32::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let mut res = Self::random();
        res = res % (diff+1f32) + x;

        Ok(Box::new(res))
    }
}
impl Randomizable for f64 {
    fn random() -> Self {
        let mut buf = [0u8; 8];
        getrandom::fill(&mut buf).expect("getting random numbers should be possible");
        f64::from_ne_bytes(buf)
    }

    fn random_range(x: Self, y: Self) -> RangeResult<Self> {
        if x > y {
            return Err(RangeError::new(Box::new(x), Box::new(y)));
        }

        let diff = y - x;
        let mut res = Self::random();
        res = res % (diff+1f64) + x;

        Ok(Box::new(res))
    }
}