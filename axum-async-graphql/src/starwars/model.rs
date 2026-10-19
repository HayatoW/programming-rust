use async_graphql::{Context, Enum, Interface, Object};

use crate::starwars::{StarWars, StarWarsChar};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Episode {
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

    async fn friends<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Character<'ctx>> {
        let star_wars = ctx.data_unchecked::<StarWars>();
        star_wars
            .friends(self.0)
            .into_iter()
            .map(|ch| {
                if ch.is_human {
                    Human(ch).into()
                } else {
                    Droid(ch).into()
                }
            })
            .collect()
    }

    async fn appears_in(&self) -> &[Episode] {
        &self.0.appears_in
    }

    async fn home_planet(&self) -> &Option<&str> {
        &self.0.home_planet
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

    async fn friends<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Character<'ctx>> {
        let star_wars = ctx.data_unchecked::<StarWars>();
        star_wars
            .friends(self.0)
            .into_iter()
            .map(|ch| {
                if ch.is_human {
                    Human(ch).into()
                } else {
                    Droid(ch).into()
                }
            })
            .collect()
    }

    async fn appears_in(&self) -> &[Episode] {
        &self.0.appears_in
    }

    async fn primary_function(&self) -> &Option<&str> {
        &self.0.primary_function
    }
}

pub struct QueryRoot;

// #[Object]
// impl QueryRoot {
//     async fn human<'a>(
//         &self,
//         ctx: &Context,
//         #[graphql(desc = "id of the human")] id: String,
//     ) -> Option<Human<'a>> {
//         // まだ書けない
//         ctx.data_unchecked::<StarWars>().human
//     }
// }

#[derive(Interface)]
#[graphql(
    field(name = "id", ty = "&str"),
    field(name = "name", ty = "&str"),
    field(name = "friends", ty = "Vec<Character<'ctx>>"),
    field(name = "appears_in", ty = "&[Episode]")
)]
pub enum Character<'a> {
    Human(Human<'a>),
    Droid(Droid<'a>),
}
