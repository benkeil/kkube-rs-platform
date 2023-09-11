use garde::Validate;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, Validate, JsonSchema)]
#[kube(
    group = "platform.benkeil.de",
    version = "v1alpha1",
    kind = "AwsAccount",
    namespaced,
    shortname = "acc"
)]
#[kube(status = "AwsAccountStatus")]
#[serde(rename_all = "camelCase")]
pub struct AwsAccountSpec {
    #[garde(skip)]
    account_id: String,
    #[garde(skip)]
    team: String,
    #[garde(skip)]
    regions: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct AwsAccountStatus {
    provisioned: bool,
}

pub const CRD_NAME: &str = "AwsAccount.platform.benkeil.de";
