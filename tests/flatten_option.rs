use deep_flatten::OptionDeepFlatten;

#[test]
fn test_flatten() {
    let x = Some(Some(Some(())));
    let flattened: Option<()> = x.deep_flatten();

    assert_eq!(flattened, Some(()));
}

#[test]
fn test_very_deep_flatten() {
    let x = Some(Some(Some(Some(Some(Some(Some(Some(Some(())))))))));
    let flattened = x.deep_flatten();

    assert_eq!(flattened, Some(()));
}
