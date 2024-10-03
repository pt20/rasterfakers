pub trait ConvertFromF64 {
    /// Converts an f64 value to the target type T.
    /// Returns a default value if the conversion is out of bounds.
    fn convert_from_f64(v: f64) -> Self;
}

impl ConvertFromF64 for u8 {
    fn convert_from_f64(v: f64) -> Self {
        if v.is_finite() && v >= 0.0 && v <= u8::MAX as f64 {
            v as u8
        } else if v < 0.0 {
            0
        } else {
            u8::MAX
        }
    }
}

impl ConvertFromF64 for u16 {
    fn convert_from_f64(v: f64) -> Self {
        if v.is_finite() && v >= 0.0 && v <= u16::MAX as f64 {
            v as u16
        } else if v < 0.0 {
            0
        } else {
            u16::MAX
        }
    }
}

impl ConvertFromF64 for i16 {
    fn convert_from_f64(v: f64) -> Self {
        if v.is_finite() && v >= i16::MIN as f64 && v <= i16::MAX as f64 {
            v as i16
        } else if v < i16::MIN as f64 {
            i16::MIN
        } else {
            i16::MAX
        }
    }
}

impl ConvertFromF64 for u32 {
    fn convert_from_f64(v: f64) -> Self {
        if v.is_finite() && v >= 0.0 && v <= u32::MAX as f64 {
            v as u32
        } else if v < 0.0 {
            0
        } else {
            u32::MAX
        }
    }
}

impl ConvertFromF64 for i32 {
    fn convert_from_f64(v: f64) -> Self {
        if v.is_finite() && v >= i32::MIN as f64 && v <= i32::MAX as f64 {
            v as i32
        } else if v < i32::MIN as f64 {
            i32::MIN
        } else {
            i32::MAX
        }
    }
}

impl ConvertFromF64 for f32 {
    fn convert_from_f64(v: f64) -> Self {
        if v.is_finite() {
            v as f32
        } else {
            f32::NAN
        }
    }
}

impl ConvertFromF64 for f64 {
    fn convert_from_f64(v: f64) -> Self {
        v
    }
}
