use chrono::{self, prelude::*};
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Orgs {
    pub orgs: Vec<Org>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Org {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub group: Option<Group>,
    pub created: Option<chrono::DateTime<Local>>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Group {
    pub name: String,
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn successfully_deserializes_json() {
        let json = r#"{
            "orgs": [
              {
                "name": "defaultOrg",
                "id": "689ce7f9-7943-4a71-b704-2ba575f01089",
                "slug": "default-org",
                "url": "https://api.snyk.io/org/default-org",
                "group": null,
                "created": "2022-09-01T19:38:22.275Z"
              },
              {
                "name": "My Other Org",
                "id": "a04d9cbd-ae6e-44af-b573-0556b0ad4bd2",
                "slug": "my-other-org",
                "url": "https://api.snyk.io/org/my-other-org",
                "group": {
                  "name": "ACME Inc.",
                  "id": "a060a49f-636e-480f-9e14-38e773b2a97f"
                }
              }
            ]
          }"#;

        let expect = Orgs {
            orgs: vec![
                Org {
                    name: String::from("defaultOrg"),
                    id: String::from("689ce7f9-7943-4a71-b704-2ba575f01089"),
                    slug: String::from("default-org"),
                    url: String::from("https://api.snyk.io/org/default-org"),
                    group: None,
                    created: Some("2022-09-01T19:38:22.275Z".parse().unwrap()),
                },
                Org {
                    name: String::from("My Other Org"),
                    id: String::from("a04d9cbd-ae6e-44af-b573-0556b0ad4bd2"),
                    slug: String::from("my-other-org"),
                    url: String::from("https://api.snyk.io/org/my-other-org"),
                    group: Some(Group {
                        name: String::from("ACME Inc."),
                        id: String::from("a060a49f-636e-480f-9e14-38e773b2a97f"),
                    }),
                    created: None,
                },
            ],
        };

        let got: Orgs = serde_json::from_str(json).unwrap();

        assert_eq!(got, expect)
    }
}
