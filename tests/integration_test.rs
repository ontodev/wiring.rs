use wiring_rs;
use wiring_rs::ofn_2_thick;
use wiring_rs::thick_2_ofn;

#[test]
fn test_intersection() {
    let ofn_intersection =
        r#"["SubClassOf","ex:intersection",["ObjectIntersectionOf","ex:I1","ex:I2","ex:I3"]]"#;

    let ofn2thick = ofn_2_thick::parser::parse(ofn_intersection);
    let ofn_str = ofn2thick.to_string();
    let thick2ofn = thick_2_ofn::parser::parse_triple(&ofn_str);
    let thick2ofn_str = thick2ofn.to_string();

    assert_eq!(ofn_intersection, thick2ofn_str);
}
