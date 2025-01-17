use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::handlers::validator::{validate_is_screamingsnake_case, validate_password_strength};

#[derive(Deserialize, Serialize, Validate)]
pub struct SignFormRequest {
    #[validate(email(message = "invalid email format"))]
    pub email: String,

    #[validate(custom(
        function = "validate_password_strength",
        message = "please enter a stronger password"
    ))]
    pub password: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub email: String,
    pub token: String,
    pub onboarded: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateOrgRequest {
    #[validate(required)]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateOrgRequest {
    #[validate(required)]
    pub id: Option<Uuid>,
    #[validate(required)]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationInfo {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing)]
    pub slug: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateOrgEnvironment {
    #[validate(required)]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgEnvironment {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrgFeatureFlag {
    #[validate(required)]
    pub env_id: Option<Uuid>,
    #[validate(custom(
        function = "validate_is_screamingsnake_case",
        message = "please insert a screaming snake case value"
    ))]
    pub name: Option<String>,
    #[validate(required)]
    pub value: Option<bool>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlag {
    pub id: Uuid,
    pub env_id: Uuid,
    pub name: String,
    pub public_name: String,
    pub description: Option<String>,
    pub value: bool,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ToggleFeatureFlagRequest {
    #[validate(required)]
    pub value: Option<bool>,
}

impl ToggleFeatureFlagRequest {
    pub fn toggle_value(&mut self) {
        self.value = Some(!self.value.unwrap())
    }
}
