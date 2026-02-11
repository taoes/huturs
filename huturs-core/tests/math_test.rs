use huturs_core::math;

#[test]
pub fn test_variance() {
    let nums = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let var = math::variance(&nums);
    assert!((var - 2.0).abs() < 0.0001);
}

#[test]
pub fn test_variance_empty() {
    let nums: Vec<f64> = vec![];
    let var = math::variance(&nums);
    assert_eq!(var, 0.0);
}

#[test]
pub fn test_variance_single() {
    let nums = vec![5.0];
    let var = math::variance(&nums);
    assert_eq!(var, 0.0);
}

#[test]
pub fn test_sample_variance() {
    let nums = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let var = math::sample_variance(&nums);
    assert!((var - 2.5).abs() < 0.0001);
}

#[test]
pub fn test_sample_variance_empty() {
    let nums: Vec<f64> = vec![];
    let var = math::sample_variance(&nums);
    assert_eq!(var, 0.0);
}

#[test]
pub fn test_sample_variance_single() {
    let nums = vec![5.0];
    let var = math::sample_variance(&nums);
    assert_eq!(var, 0.0);
}

#[test]
pub fn test_standard_deviation() {
    let nums = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let std_dev = math::standard_deviation(&nums);
    assert!((std_dev - 1.4142).abs() < 0.0001);
}

#[test]
pub fn test_standard_deviation_empty() {
    let nums: Vec<f64> = vec![];
    let std_dev = math::standard_deviation(&nums);
    assert_eq!(std_dev, 0.0);
}

#[test]
pub fn test_standard_deviation_single() {
    let nums = vec![5.0];
    let std_dev = math::standard_deviation(&nums);
    assert_eq!(std_dev, 0.0);
}

#[test]
pub fn test_sample_standard_deviation() {
    let nums = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let std_dev = math::sample_standard_deviation(&nums);
    assert!((std_dev - 1.5811).abs() < 0.0001);
}

#[test]
pub fn test_sample_standard_deviation_empty() {
    let nums: Vec<f64> = vec![];
    let std_dev = math::sample_standard_deviation(&nums);
    assert_eq!(std_dev, 0.0);
}

#[test]
pub fn test_sample_standard_deviation_single() {
    let nums = vec![5.0];
    let std_dev = math::sample_standard_deviation(&nums);
    assert_eq!(std_dev, 0.0);
}