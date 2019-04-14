use UnscaledMeasurement;

pub fn convert_8bit(x: u8, y: u8, z: u8) -> UnscaledMeasurement {
    UnscaledMeasurement {
        x: i16::from(x as i8),
        y: i16::from(y as i8),
        z: i16::from(z as i8),
    }
}

pub fn convert_12bit(x: u16, y: u16, z: u16) -> UnscaledMeasurement {
    UnscaledMeasurement {
        x: (x << 4) as i16 / 16,
        y: (y << 4) as i16 / 16,
        z: (z << 4) as i16 / 16,
    }
}

pub fn convert_14bit(x: u16, y: u16, z: u16) -> UnscaledMeasurement {
    UnscaledMeasurement {
        x: (x << 2) as i16 / 4,
        y: (y << 2) as i16 / 4,
        z: (z << 2) as i16 / 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_positive_8bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: 1,
                y: 64,
                z: 127
            },
            convert_8bit(1, 64, 127)
        );
    }
    #[test]
    fn can_convert_negative_8bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: -1,
                y: -64,
                z: -128
            },
            convert_8bit(0b1111_1111, 0b1100_0000, 0b1000_0000)
        );
    }

    #[test]
    fn can_convert_positive_12bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: 1,
                y: 1024,
                z: 2047
            },
            convert_12bit(1, 1024, 2047)
        );
    }
    #[test]
    fn can_convert_negative_12bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: -1,
                y: -1024,
                z: -2048
            },
            convert_12bit(0b1111_1111_1111, 0b1100_0000_0000, 0b1000_0000_0000)
        );
    }

    #[test]
    fn can_convert_positive_14bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: 1,
                y: 4096,
                z: 8191
            },
            convert_14bit(1, 4096, 8191)
        );
    }
    #[test]
    fn can_convert_negative_14bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: -1,
                y: -4096,
                z: -8192
            },
            convert_14bit(
                0b11_1111_1111_1111,
                0b11_0000_0000_0000,
                0b10_0000_0000_0000
            )
        );
    }
}
