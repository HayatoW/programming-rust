pub use std::boxed::Box;
pub use std::collections::HashMap;
pub use std::string::ToString;

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
    ($other:tt) => {
        $crate::Json::from($other)
    };
}

#[cfg(test)]
mod test {
    use crate::Json;

    #[test]
    fn json_with_rust_expressions() {
        const HELLO: &'static str = "hello";
        let macro_generated_value = json!({
            "math_works": (4 - 2 == 2),
            "en": HELLO,
            HELLO: "bonjour!"
        });
        let hand_coded_value = Json::Object(Box::new(
            vec![
                ("math_works".to_string(), Json::Boolean(true)),
                ("en".to_string(), Json::String("hello".to_string())),
                ("hello".to_string(), Json::String("bonjour!".to_string())),
            ]
            .into_iter()
            .collect(),
        ));
        assert_eq!(macro_generated_value, hand_coded_value);
    }
}
