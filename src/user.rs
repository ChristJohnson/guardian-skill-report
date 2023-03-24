use super::*;

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// This contract supplies basic information commonly used to display a minimal
/// amount of information about a user. Take care to not add more properties
/// here unless the property applies in all (or at least the majority) of the
/// situations where UserInfoCard is used. Avoid adding game specific or
/// platform specific details here. In cases where UserInfoCard is a subset of
/// the data needed in a contract, use UserInfoCard as a property of other
/// contracts.
///
/// Read more at:
/// https://bungie-net.github.io/multi/schema_User-UserInfoCard.html#schema_User-UserInfoCard
#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct UserInfoCard {
    // A platform specific additional display name - ex: psn Real Name, bnet Unique Name, etc.
    supplementalDisplayName: String,
    // URL the Icon if available.
    iconPath: String,
    // If there is a cross save override in effect, this value will tell you the type that is overridding this one.
    crossSaveOverride: i32,
    // The list of Membership Types indicating the platforms on which this Membership can be used.
    // Not in Cross Save = its original membership type. Cross Save Primary = Any membership types it is overridding, and its original membership type Cross Save Overridden = Empty list
    applicableMembershipTypes: [i32; 8],
    // If True, this is a public user membership.
    isPublic: bool,
    // Type of the membership. Not necessarily the native type.
    membershipType: i32,
    // Membership ID as they user is known in the Accounts service
    membershipId: i64,
    // Display Name the player has chosen for themselves. The display name is optional when the data type is used as input to a platform API.
    displayName: String,
    // The bungie global display name, if set.
    bungieGlobalDisplayName: String,
    // The bungie global display name code, if set.
    // WARN: CAN BE NULL
    bungieGlobalDisplayNameCode: i16,
}

impl From<String> for UserInfoCard {
    fn from(_value: String) -> Self {
        todo!()
    }
}

impl From<serde_json::Value> for UserInfoCard {
    fn from(_value: serde_json::Value) -> Self {
        todo!()
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) struct ExactSearchRequest {
    pub(crate) displayName: String,
    pub(crate) displayNameCode: i16,
}

pub(crate) async fn _bungie_user_search(
    client: &Client,
    path: &str,
    search_parameter: &ExactSearchRequest,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let search_string = serde_json::to_string(search_parameter)?;
    let player = client
        .post(_BUNGIE_ROOT.to_owned() + path)
        .body(search_string)
        .send()
        .await?;

    Ok(player)
}