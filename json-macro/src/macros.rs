#[macro_export]
macro_rules! json {
    (null) => {
        $create::Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        $crate::Json::Array(vec![ $( json!($element) ),* ])
    };
}
