use wiring_rs;
use wiring_rs::thick2ofn as thick2ofn;
use wiring_rs::ofn2thick as ofn2thick;

#[test]
fn test_intersection() {

    let ofn_intersection =  r#"["SubClassOf","ex:intersection",["ObjectIntersectionOf","ex:I1","ex:I2","ex:I3"]]"#;

    let ofn2thick = ofn2thick::ofn_parser::parse_ofn(ofn_intersection);
    let thick2ofn = thick2ofn::thick_triple_parser::parse_tiple(&ofn2thick); 

    assert_eq!(ofn_intersection, thick2ofn);

}
