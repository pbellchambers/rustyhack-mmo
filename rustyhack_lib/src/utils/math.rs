#[must_use]
pub fn i32_from(value: u32) -> i32 {
    if let Ok(converted_value) = i32::try_from(value) {
        converted_value
    } else {
        warn!(
            "u32 value: {} could not be converted to i32, most likely due to overflow/wrapping.",
            &value
        );
        //return zero
        0
    }
}

#[must_use]
pub fn i32_from_usize(value: usize) -> i32 {
    if let Ok(converted_value) = i32::try_from(value) {
        converted_value
    } else {
        warn!(
            "Usize value: {} could not be converted to i32, most likely due to overflow/wrapping.",
            &value
        );
        //return zero
        0
    }
}

#[must_use]
pub fn usize_from_i32(value: i32) -> usize {
    if let Ok(converted_value) = usize::try_from(value) {
        converted_value
    } else {
        warn!(
            "i32 value: {} could not be converted to Usize, most likely due to being negative.",
            &value
        );
        //return zero
        0
    }
}

#[must_use]
pub fn u32_from(value: i32) -> u32 {
    if let Ok(converted_value) = u32::try_from(value) {
        converted_value
    } else {
        warn!(
            "i32 value: {} could not be converted to u32, most likely due to being negative.",
            &value
        );
        //return zero
        0
    }
}
