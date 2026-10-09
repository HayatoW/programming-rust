use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Epsode {
    /// 1997年公開
    NewHope,
    /// 1980年公開
    Empire,
    /// 1983年公開
    Jedi,
}

pub struct Human();
