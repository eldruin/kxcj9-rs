use {ic, private, Error, GScaleConfig, Measurement, MeasurementBits, UnscaledMeasurement};

#[doc(hidden)]
pub trait ScaledDevice: private::Sealed {
    fn get_scaled(
        unscaled: UnscaledMeasurement,
        bits: MeasurementBits,
        scale_config: GScaleConfig,
    ) -> Measurement;

    fn get_wake_up_threshold<E>(threshold: f32) -> Result<u8, Error<E>>;
}

impl ScaledDevice for ic::Kxcj9_1008 {
    fn get_scaled(
        unscaled: UnscaledMeasurement,
        bits: MeasurementBits,
        scale_config: GScaleConfig,
    ) -> Measurement {
        let max = bits.max();
        let g = match scale_config {
            GScaleConfig::_0 => 2.0,
            GScaleConfig::_1 => 4.0,
            GScaleConfig::_2 => 8.0,
            GScaleConfig::_3 => 8.0,
        };
        Measurement {
            x: f32::from(unscaled.x) * g / max,
            y: f32::from(unscaled.y) * g / max,
            z: f32::from(unscaled.z) * g / max,
        }
    }

    fn get_wake_up_threshold<E>(threshold: f32) -> Result<u8, Error<E>> {
        if threshold < 0.0 || threshold > 8.0 {
            Err(Error::InvalidSetting)
        } else {
            Ok((threshold * 16.0 + 0.5) as u8)
        }
    }
}

impl ScaledDevice for ic::Kxcj9_1018 {
    fn get_scaled(
        unscaled: UnscaledMeasurement,
        bits: MeasurementBits,
        scale_config: GScaleConfig,
    ) -> Measurement {
        let max = bits.max();
        let g = match scale_config {
            GScaleConfig::_0 => 4.0,
            GScaleConfig::_1 => 8.0,
            GScaleConfig::_2 => 16.0,
            GScaleConfig::_3 => 16.0,
        };
        Measurement {
            x: f32::from(unscaled.x) * g / max,
            y: f32::from(unscaled.y) * g / max,
            z: f32::from(unscaled.z) * g / max,
        }
    }

    fn get_wake_up_threshold<E>(threshold: f32) -> Result<u8, Error<E>> {
        // There is a mismatch in the datasheet for the KXCJ9-1018 model.
        // Kionix engineers confirmed me that the reset value corresponds to 1g.
        if threshold < 0.0 || threshold > 16.0 {
            Err(Error::InvalidSetting)
        } else {
            Ok((threshold * 8.0 + 0.5) as u8)
        }
    }
}

#[cfg(test)]
mod tests_scale {
    use super::*;

    fn assert_near_positive(a: f32, b: f32) {
        if (a - b) > 0.5 || (b - a) > 0.5 {
            panic!();
        }
    }

    #[test]
    #[should_panic]
    fn assert_near_can_fail() {
        assert_near_positive(1.0, 2.0)
    }

    macro_rules! test {
        ($name:ident, $ic:ident, $max:expr, $bits:ident, $scale:ident, $expected_max:expr) => {
            #[test]
            fn $name() {
                let unscaled = UnscaledMeasurement {
                    x: 0,
                    y: $max / 2,
                    z: $max,
                };
                let m = ic::$ic::get_scaled(unscaled, MeasurementBits::$bits, GScaleConfig::$scale);
                assert_near_positive(m.x, 0.0);
                assert_near_positive(m.y, $expected_max / 2.0);
                assert_near_positive(m.z, $expected_max);
            }
        };
    }

    test!(scale_1008_8b_2g, Kxcj9_1008, 127, _8bit, _0, 2.0);
    test!(scale_1008_8b_4g, Kxcj9_1008, 127, _8bit, _1, 4.0);
    test!(scale_1008_8b_8g, Kxcj9_1008, 127, _8bit, _2, 8.0);

    test!(scale_1008_12b_2g, Kxcj9_1008, 2047, _12bit, _0, 2.0);
    test!(scale_1008_12b_4g, Kxcj9_1008, 2047, _12bit, _1, 4.0);
    test!(scale_1008_12b_8g, Kxcj9_1008, 2047, _12bit, _2, 8.0);
    test!(scale_1008_12b_8gfp, Kxcj9_1008, 2047, _12bit, _3, 8.0);

    test!(scale_1008_14b_2g, Kxcj9_1008, 8191, _14bit, _0, 2.0);
    test!(scale_1008_14b_4g, Kxcj9_1008, 8191, _14bit, _1, 4.0);
    test!(scale_1008_14b_8g, Kxcj9_1008, 8191, _14bit, _2, 8.0);
    test!(scale_1008_14b_8gfp, Kxcj9_1008, 8191, _14bit, _3, 8.0);

    test!(scale_1018_8b_4g, Kxcj9_1018, 127, _8bit, _0, 4.0);
    test!(scale_1018_8b_8g, Kxcj9_1018, 127, _8bit, _1, 8.0);
    test!(scale_1018_8b_16g, Kxcj9_1018, 127, _8bit, _2, 16.0);

    test!(scale_1018_12b_4g, Kxcj9_1018, 2047, _12bit, _0, 4.0);
    test!(scale_1018_12b_8g, Kxcj9_1018, 2047, _12bit, _1, 8.0);
    test!(scale_1018_12b_16g, Kxcj9_1018, 2047, _12bit, _2, 16.0);
    test!(scale_1018_12b_16gfp, Kxcj9_1018, 2047, _12bit, _3, 16.0);

    test!(scale_1018_14b_4g, Kxcj9_1018, 8191, _14bit, _0, 4.0);
    test!(scale_1018_14b_8g, Kxcj9_1018, 8191, _14bit, _1, 8.0);
    test!(scale_1018_14b_16g, Kxcj9_1018, 8191, _14bit, _2, 16.0);
    test!(scale_1018_14b_16gfp, Kxcj9_1018, 8191, _14bit, _3, 16.0);
}

#[cfg(test)]
mod tests_wake_up_threshold {
    use super::*;

    macro_rules! test {
        ($name:ident, $ic:ident, $threshold:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let threshold = ic::$ic::get_wake_up_threshold::<()>($threshold).unwrap();
                assert_eq!($expected, threshold);
            }
        };
    }
    macro_rules! test_fail {
        ($name:ident, $ic:ident, $threshold:expr) => {
            #[test]
            fn $name() {
                ic::$ic::get_wake_up_threshold::<()>($threshold).expect_err("Should fail");
            }
        };
    }

    test_fail!(cannot_set_1008_too_big, Kxcj9_1008, 8.1);
    test_fail!(cannot_set_1008_negative, Kxcj9_1008, -0.1);
    test!(th_1008_min, Kxcj9_1008, 0.0, 0);
    test!(th_1008_0_5, Kxcj9_1008, 0.5, 8);
    test!(th_1008_max, Kxcj9_1008, 8.0, 128);

    test_fail!(cannot_set_1018_too_big, Kxcj9_1018, 16.1);
    test_fail!(cannot_set_1018_negative, Kxcj9_1018, -0.1);
    test!(th_1018_min, Kxcj9_1018, 0.0, 0);
    test!(th_1018_0_5, Kxcj9_1018, 0.5, 4);
    test!(th_1018_max, Kxcj9_1018, 16.0, 128);
}
