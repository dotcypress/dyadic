use dyadic::DF;

#[test]
fn test_add() {
    let a = DF::from(2);
    let b = DF::new(4, 3);
    let c = a + b;
    let d = c * 1000.into();
    assert_eq!(2500, d.try_into().unwrap())
}

#[test]
fn test_sub() {
    let a = DF::from(2);
    let b = DF::new(4, 3);
    let c = a - b;
    let d = c * 1000.into();
    assert_eq!(1500, d.try_into().unwrap())
}

#[test]
fn test_mul() {
    let mut a = DF::new(3, 2);
    a *= 100.into();
    assert_eq!(75, a.try_into().unwrap())
}

#[test]
fn test_neg() {
    let a = DF::new(3, 2);
    let b = DF::new(-3, 2);
    assert_eq!(a, -b)
}

#[test]
fn test_abs() {
    let a = DF::new(3, 2);
    let b = DF::new(-3, 2);
    assert_eq!(a, b.abs())
}

#[test]
fn test_eq() {
    let a = DF::new(4, 3);
    let b = DF::new(8, 4);
    assert_eq!(a, b)
}

#[test]
fn test_cmp() {
    let a = DF::new(4, 3);
    let b = DF::new(7, 4);
    assert!(a > b)
}

#[test]
fn test_max() {
    let a = DF::new(4, 3);
    let b = DF::new(7, 4);
    let min = DF::min(a, b);
    let max = DF::max(a, b);
    assert_eq!(a, max);
    assert_eq!(b, min);
}

#[test]
fn test_copysign() {
    let a = DF::new(4, 3);
    let b = DF::new(-7, 4);

    assert_eq!(a.copysign(42), a);
    assert_eq!(a.copysign(-42), -a);
    assert_eq!(b.copysign(42), -b);
    assert_eq!(b.copysign(-42), b);
}

#[test]
fn test_pow() {
    let a = DF::new(3, 3);
    let b = a.pow(4);
    assert_eq!(b, DF::new(81, 12));
}

#[test]
fn test_scale() {
    let a = DF::new(3, 3);
    assert_eq!(a.scale(1000), 375);
    assert_eq!(a.scale(100), 37);
}
