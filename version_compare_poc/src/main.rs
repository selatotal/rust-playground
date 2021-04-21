use version_compare::{CompOp, VersionCompare};


fn main() {

    // Define some version numbers
    let a = "1.2";
    let b = "1.5.1";

    // Easily compare version strings
    assert_eq!(VersionCompare::compare(&a, &b).unwrap(), CompOp::Lt);
    assert_eq!(VersionCompare::compare_to(&a, &b, &CompOp::Le).unwrap(), true);
    assert_eq!(VersionCompare::compare_to("", &b, &CompOp::Le).unwrap(), true);
    assert_eq!(VersionCompare::compare_to(&String::default(), &b, &CompOp::Le).unwrap(), true);
    assert_eq!(VersionCompare::compare_to(&a, &b, &CompOp::Gt).unwrap(), false);

}
