use async_graphql::{Enum, Object};

use crate::starwars::StarWarsChar;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Epsode {
    /// 1997年公開
    NewHope,
    /// 1980年公開
    Empire,
    /// 1983年公開
    Jedi,
}

pub struct Human<'a>(&'a StarWarsChar);

#[Object]
impl<'a> Human<'a> {
    async fn id(&self) -> &str {
        self.0.id
    }

    async fn name(&self) -> &str {
        self.0.name
    }
}

pub struct Droid<'a>(&'a StarWarsChar);

#[Object]
impl<'a> Droid<'a> {
    async fn id(&self) -> &str {
        self.0.id
    }

    async fn name(&self) -> &str {
        self.0.name
    }
}
