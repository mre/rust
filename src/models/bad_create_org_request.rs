/*
 * propelauth
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BadCreateOrgRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<String>>,
}

impl BadCreateOrgRequest {
    pub fn new() -> BadCreateOrgRequest {
        BadCreateOrgRequest {
            name: None,
        }
    }
}


