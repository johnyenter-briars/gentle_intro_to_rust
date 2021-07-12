#[macro_use]
extern crate json;

fn main() {
    let doc = json::parse(r#"
    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }
    "#).expect("parse failed!");

    println!("{:?}", doc);
    println!("{}", doc);

    let code = doc["code"].as_u32().unwrap_or(0);
    let success = doc["success"].as_bool().unwrap_or(false);

    assert_eq!(code, 200);
    assert_eq!(success, true);

    //has to be a reference - otherwise we'd be MOVING the value out of document (doesnt implement copy trait remember)
    let features = &doc["payload"]["features"];

    for v in features.members() {
        println!("{}", v.as_str().unwrap())
    }

    let data = object!{
        "name" => "John YB",
        "age" => 30,
        "numbers" => array![10, 20, 30]
    };

    assert_eq!(
        data.dump(),
        r#"{"name":"John YB","age":30,"numbers":[10,20,30]}"#
    );


}
