use async_graphql::{dynamic::*, Value};

use crate::{Episode, StarWars, StarWarsChar};

impl<'a> From<Episode> for FieldValue<'a> {
    fn from(value: Episode) -> Self {
        match value {
            Episode::NewHope => FieldValue::value("NEW_HOPE"),
            Episode::Empire => FieldValue::value("EMPIRE"),
            Episode::Jedi => FieldValue::value("JEDI"),
        }
    }
}

pub fn schema() -> Result<Schema, SchemaError> {
    let episode = Enum::new("Episode")
        .item(EnumItem::new("NEW_HOPE").description("Released in 1977."))
        .item(EnumItem::new("EMPIRE").description("Released in 1980."))
        .item(EnumItem::new("JEDI").description("Released in 1983."));

    let character = Interface::new("Character")
        .field(InterfaceField::new(
            "id",
            TypeRef::named_nn(TypeRef::STRING),
        ))
        .field(InterfaceField::new(
            "name",
            TypeRef::named_nn(TypeRef::STRING),
        ))
        .field(InterfaceField::new(
            "friends",
            TypeRef::named_nn_list_nn("Character"),
        ))
        .field(InterfaceField::new(
            "appearsIn",
            TypeRef::named_nn_list_nn(episode.type_name()),
        ));

    let human = Object::new("Human")
        .description("A humanoid creature in the Star Wars universe.")
        .implement(character.type_name())
        .field(
            Field::new("id", TypeRef::named_nn(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                    Ok(Some(Value::from(char.id)))
                })
            })
            .description("The id of the human."),
        )
        .field(
            Field::new("name", TypeRef::named_nn(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                    Ok(Some(Value::from(char.name)))
                })
            })
            .description("The name of the human."),
        )
        .field(
            Field::new(
                "friends",
                TypeRef::named_nn_list_nn(character.type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                        let starwars = ctx.data::<StarWars>()?;
                        let friends = starwars.friends(char);
                        Ok(Some(FieldValue::list(friends.into_iter().map(|friend| {
                            FieldValue::borrowed_any(friend).with_type(if friend.is_human {
                                "Human"
                            } else {
                                "Droid"
                            })
                        }))))
                    })
                },
            )
            .description("The friends of the human, or an empty list if they have none."),
        )
        .field(
            Field::new(
                "appearsIn",
                TypeRef::named_nn_list_nn(episode.type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                        Ok(Some(FieldValue::list(
                            char.appears_in.iter().copied().map(FieldValue::from),
                        )))
                    })
                },
            )
            .description("Which movies they appear in."),
        )
        .field(
            Field::new("homePlanet", TypeRef::named(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                    Ok(char.home_planet.map(Value::from))
                })
            })
            .description("The home planet of the human, or null if unknown."),
        );

    let droid = Object::new("Droid")
        .description("A mechanical creature in the Star Wars universe.")
        .implement(character.type_name())
        .field(
            Field::new("id", TypeRef::named_nn(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                    Ok(Some(Value::from(char.id)))
                })
            })
            .description("The id of the droid."),
        )
        .field(
            Field::new("name", TypeRef::named_nn(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                    Ok(Some(Value::from(char.name)))
                })
            })
            .description("The name of the droid."),
        )
        .field(
            Field::new(
                "friends",
                TypeRef::named_nn_list_nn(character.type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                        let starwars = ctx.data::<StarWars>()?;
                        let friends = starwars.friends(char);
                        Ok(Some(FieldValue::list(friends.into_iter().map(|friend| {
                            FieldValue::borrowed_any(friend).with_type(if friend.is_human {
                                "Human"
                            } else {
                                "Droid"
                            })
                        }))))
                    })
                },
            )
            .description("The friends of the droid, or an empty list if they have none."),
        )
        .field(
            Field::new(
                "appearsIn",
                TypeRef::named_nn_list_nn(episode.type_name()),
                |ctx| {
                    FieldFuture::new(async move {
                        let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                        Ok(Some(FieldValue::list(
                            char.appears_in.iter().copied().map(FieldValue::from),
                        )))
                    })
                },
            )
            .description("Which movies they appear in."),
        )
        .field(
            Field::new("primaryFunction", TypeRef::named(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let char = ctx.parent_value.try_downcast_ref::<StarWarsChar>()?;
                    Ok(char.primary_function.map(Value::from))
                })
            })
            .description("The primary function of the droid."),
        );

    let query = Object::new("Qurey")
        .field(
            Field::new("hero", TypeRef::named_nn(character.type_name()), |ctx| {
                FieldFuture::new(async move {
                    let starwars = ctx.data::<StarWars>()?;
                    let episode = match ctx.args.get("episode") {
                        Some(episode) => Some(match episode.enum_name()? {
                            "NEW_HOPE" => Episode::NewHope,
                            "EMPIRE" => Episode::Empire,
                            "JEDI" => Episode::Jedi,
                            _ => unreachable!(),
                        }),
                        None => None,
                    };
                    let value = match episode {
                        Some(episode) => {
                            if episode == Episode::Empire {
                                FieldValue::borrowed_any(starwars.chars.get(starwars.luke).unwrap())
                                    .with_type("Human")
                            } else {
                                FieldValue::borrowed_any(
                                    starwars.chars.get(starwars.artoo).unwrap(),
                                )
                                .with_type("Droid")
                            }
                        }
                        None => {
                            FieldValue::borrowed_any(starwars.chars.get(starwars.luke).unwrap())
                                .with_type("Human")
                        }
                    };
                    Ok(Some(value))
                })
            })
            .argument(InputValue::new(
                "episode",
                TypeRef::named(episode.type_name()),
            )),
        )
        .field(
            Field::new("human", TypeRef::named(human.type_name()), |ctx| {
                FieldFuture::new(async move {
                    let starwars = ctx.data::<StarWars>()?;
                    let id = ctx.args.try_get("id")?;
                    Ok(starwars
                        .human(id.string()?)
                        .map(|v| FieldValue::borrowed_any(v)))
                })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::STRING))),
        )
        .field(Field::new(
            "humans",
            TypeRef::named_nn_list_nn(human.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let starwars = ctx.data::<StarWars>()?;
                    let humans = starwars.humans();
                    Ok(Some(FieldValue::list(
                        humans.into_iter().map(|v| FieldValue::borrowed_any(v)),
                    )))
                })
            },
        ))
        .field(
            Field::new("droid", TypeRef::named(human.type_name()), |ctx| {
                FieldFuture::new(async move {
                    let starwars = ctx.data::<StarWars>()?;
                    let id = ctx.args.try_get("id")?;
                    Ok(starwars
                        .droid(id.string()?)
                        .map(|v| FieldValue::borrowed_any(v)))
                })
            })
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::STRING))),
        )
        .field(Field::new(
            "droids",
            TypeRef::named_nn_list_nn(human.type_name()),
            |ctx| {
                FieldFuture::new(async move {
                    let starwars = ctx.data::<StarWars>()?;
                    let droids = starwars.droids();
                    Ok(Some(FieldValue::list(
                        droids.into_iter().map(|v| FieldValue::borrowed_any(v)),
                    )))
                })
            },
        ));

    Schema::build(query.type_name(), None, None)
        .register(episode)
        .register(character)
        .register(human)
        .register(droid)
        .register(query)
        .data(StarWars::new())
        .finish()
}
