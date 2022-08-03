use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}
