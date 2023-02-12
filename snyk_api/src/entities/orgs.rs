use serde::Serialize;
use snyk_data;

pub trait FromModel {
    fn from_model(model: snyk_data::model::org::Orgs) -> Self;
}

pub type Orgs = Vec<Org>;

impl FromModel for Orgs {
    fn from_model(model: snyk_data::model::org::Orgs) -> Self {
        model.orgs.into_iter()
            .map(|org| {
                Org {
                    name: org.name,
                    id: org.id
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Org {
    pub name: String,
    pub id: String
}