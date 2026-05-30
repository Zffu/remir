#[derive(Debug, Clone, Copy)]
pub enum ConstantData {
    Int(i128),
    Float(f64),
    Pointer(usize),
    None,
}

impl ConstantData {
    pub fn as_int(self) -> i128 {
        match self {
            Self::Int(v) => v,
            _ => panic!("Expected Self::Int got {:#?}", self),
        }
    }

    pub fn as_float(self) -> f64 {
        match self {
            Self::Float(v) => v,
            _ => panic!("Expected Self::Float got {:#?}", self),
        }
    }

    pub fn as_ptr(self) -> usize {
        match self {
            Self::Pointer(v) => v,
            _ => panic!("Expected Self::Pointer got {:#?}", self),
        }
    }
}
