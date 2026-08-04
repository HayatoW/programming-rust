#[macro_export]
macro_rules! json {
    (null) => {
        $create::Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        $crate::Json::Array(vec![ $( json!($element) ),* ])
    };
    ({$($key:tt : $value: tt), *}) => {
        {
            let mut fields = $crate::macros::Box::new($crate::macros::HashMap::new());
            $(fields.insert($crate::macros::ToString::to_string($key), json!($value));)*
            $crate::Json::Object(fields)
        }
    };
}
