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
pub struct FetchOrgsResponse {
    #[serde(rename = "orgs")]
    pub orgs: Vec<crate::models::FetchOrgResponse>,
    #[serde(rename = "total_orgs")]
    pub total_orgs: i64,
    #[serde(rename = "current_page")]
    pub current_page: i64,
    #[serde(rename = "page_size")]
    pub page_size: i64,
    #[serde(rename = "has_more_results")]
    pub has_more_results: bool,
}

impl FetchOrgsResponse {
    pub fn new(orgs: Vec<crate::models::FetchOrgResponse>, total_orgs: i64, current_page: i64, page_size: i64, has_more_results: bool) -> FetchOrgsResponse {
        FetchOrgsResponse {
            orgs,
            total_orgs,
            current_page,
            page_size,
            has_more_results,
        }
    }
}


