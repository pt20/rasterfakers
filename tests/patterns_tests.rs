use rasterfakers::patterns::{DataGenerator, GradientPattern, NoisePattern, SineWavePattern};

#[test]
fn test_gradient_pattern() {
    let pattern = GradientPattern;

    // Test basic functionality
    assert_eq!(pattern.generate(0, 0, 0), 0.0);
    assert_eq!(pattern.generate(1, 1, 1), 3.0);
    assert_eq!(pattern.generate(10, 20, 3), 33.0);

    // Test large numbers
    assert_eq!(pattern.generate(1000, 2000, 5), 3005.0);

    // Test zero values
    assert_eq!(pattern.generate(0, 0, 1), 1.0);
}

#[test]
fn test_sine_wave_pattern() {
    let pattern = SineWavePattern;

    // Test basic functionality
    let value1 = pattern.generate(0, 0, 0);
    let value2 = pattern.generate(25, 25, 1);

    // Values should be different
    assert!(value1 != value2);

    // Values should be within the expected range [0, 255]
    assert!(value1 >= 0.0 && value1 <= 255.0, "value1 is {}", value1);
    assert!(value2 >= 0.0 && value2 <= 255.0, "value2 is {}", value2);

    // Test that different bands produce different values
    let value3 = pattern.generate(50, 50, 0);
    let value4 = pattern.generate(50, 50, 1);
    assert!(value3 != value4);

    // Test extreme values
    let mut max_value = f64::NEG_INFINITY;
    let mut min_value = f64::INFINITY;

    for x in 0..1000 {
        for y in 0..1000 {
            let value = pattern.generate(x, y, 0);
            max_value = max_value.max(value);
            min_value = min_value.min(value);
        }
    }

    assert!(max_value <= 255.0, "Max value is {}", max_value);
    assert!(min_value >= 0.0, "Min value is {}", min_value);
}

#[test]
fn test_noise_pattern() {
    let pattern = NoisePattern;

    // Test basic functionality
    let value1 = pattern.generate(0, 0, 0);
    let value2 = pattern.generate(1, 1, 1);

    // Values should be different
    assert!(value1 != value2);

    // Test reproducibility
    let value3 = pattern.generate(10, 20, 3);
    let value4 = pattern.generate(10, 20, 3);
    assert_eq!(value3, value4);

    // Test that different inputs produce different outputs
    let value5 = pattern.generate(100, 200, 1);
    let value6 = pattern.generate(100, 200, 2);
    assert!(value5 != value6);
}

#[test]
fn test_custom_pattern() {
    struct CustomPattern;
    impl DataGenerator for CustomPattern {
        fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
            (x * y * band * 2) as f64
        }
    }

    let pattern = CustomPattern;

    assert_eq!(pattern.generate(2, 3, 4), 48.0);
    assert_eq!(pattern.generate(0, 5, 2), 0.0);
    assert_eq!(pattern.generate(10, 10, 1), 200.0);
}
