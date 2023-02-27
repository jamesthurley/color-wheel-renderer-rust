pub enum BucketDirection {
    Down,
    Up,
}

pub fn bucket(value: f32, maximum: f32, buckets: usize, direction: BucketDirection) -> f32 {
    match direction {
        BucketDirection::Down => bucket_down(value, maximum, buckets),
        BucketDirection::Up => bucket_up(value, maximum, buckets),
    }
}

fn bucket_down(value: f32, maximum: f32, buckets: usize) -> f32 {
    if buckets < 1 {
        return value;
    }

    let factor = maximum / buckets as f32;

    (value / factor).floor() * factor
}

fn bucket_up(value: f32, maximum: f32, buckets: usize) -> f32 {
    maximum - bucket_down(maximum - value, maximum, buckets)
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn bucket_up() {
        assert_approx_eq!(f32, bucket(0.45, 1., 10, BucketDirection::Up), 0.5);
        assert_approx_eq!(f32, bucket(0.5, 1., 10, BucketDirection::Up), 0.5);
        assert_approx_eq!(f32, bucket(0.55, 1., 10, BucketDirection::Up), 0.6);
        assert_approx_eq!(f32, bucket(0.59, 1., 10, BucketDirection::Up), 0.6);
        assert_approx_eq!(f32, bucket(0.61, 1., 10, BucketDirection::Up), 0.7);

        assert_approx_eq!(f32, bucket(187., 360., 36, BucketDirection::Up), 190.);
    }

    #[test]
    fn bucket_down() {
        assert_approx_eq!(f32, bucket(0.45, 1., 10, BucketDirection::Down), 0.4);
        assert_approx_eq!(f32, bucket(0.5, 1., 10, BucketDirection::Down), 0.5);
        assert_approx_eq!(f32, bucket(0.55, 1., 10, BucketDirection::Down), 0.5);
        assert_approx_eq!(f32, bucket(0.59, 1., 10, BucketDirection::Down), 0.5);
        assert_approx_eq!(f32, bucket(0.61, 1., 10, BucketDirection::Down), 0.6);

        assert_approx_eq!(f32, bucket(187., 360., 36, BucketDirection::Down), 180.);
    }
}
