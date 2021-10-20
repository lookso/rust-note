use json::{array, object};
pub fn json_func() {
    let parsed = json::parse(
        r#"
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
    "#,
    )
    .unwrap();

    let instantiated = object! {
        "code" => 200,
        "success" => true,
        "payload" => object!{
            "features" => array![
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };
    assert_eq!(parsed, instantiated);
    println!("解析输出字段：code={}", instantiated["code"]);
    println!("解析输出字段：success={}", instantiated["success"]);
    println!("解析输出对象：payload={:?}", instantiated["payload"]);
    println!(
        "解析输出数组：features={:?}",
        instantiated["payload"]["features"]
    );
    println!(
        "解析输出数组元素：0={}",
        instantiated["payload"]["features"][0]
    );
    println!(
        "解析输出数组元素：1={}",
        instantiated["payload"]["features"][1]
    );
    println!(
        "解析输出数组元素：2={}",
        instantiated["payload"]["features"][2]
    );

    let features_0: Option<String> = Some("美貌".to_string());
    let features_1: Option<String> = None;
    let score = vec![67, 78, 87];
    let score_other = vec![Some(87), Some(89), None];
    let hobby = json::Null;

    let mut sub_items = json::JsonValue::new_object();
    sub_items["跳高"] = 2.into();
    sub_items["跳远"] = 3.into();

    let mut subject = json::JsonValue::new_array();
    subject.push(100);
    subject.push(99);
    subject.push(sub_items);

    let data = array!["123", true, json::Null, 300];
    let students = object! {
        "name" => "zhangsan",
        "sex" => 15,
        "height" => 156,
        "weight" => 45,
        "hobby1" => "吹牛逼".to_string(),
        "hobby2" => hobby,
        "ke_mu" => subject,
        "features" => array![features_0,features_1],
        "score_main" => score,
        "score_branch" => score_other,
        "others"=> data
    };
    let response = students.dump();
    println!("[返回数据]：{}", response);
}
