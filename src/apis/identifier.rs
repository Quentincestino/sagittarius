use serde::{Deserialize, Serialize};

/// The client has many ways to try to auh, so we handle them.
#[derive(Deserialize)]
pub struct UserIdentifier {
    r#type: String,
    // When type is "m.id.user", the request contains the field "user"
    user: Option<String>,
    // When type is "m.id.thirdparty", the request contains the fields "medium" and "address"
    medium: Option<String>,
    address: Option<String>,
    // When type is "m.id.phone", the request contains the fields "country" and "phone"
    country: Option<String>,
    phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MIdUserIdentifier {
    user: String,
}

#[derive(Serialize, Deserialize)]
pub struct MIdThirdpartyIdentifier {
    medium: String,
    address: String,
}

#[derive(Serialize, Deserialize)]
pub struct MIdPhoneIdentifier {
    country: String,
    phone: String,
}

pub enum IdentifierType {
    MIdUser(MIdUserIdentifier),
    MIdThirdparty(MIdThirdpartyIdentifier),
    MIdPhone(MIdPhoneIdentifier),
    None,
}

pub fn decode_identifier_type(identifier_type: &UserIdentifier) -> IdentifierType {
    match identifier_type.r#type.as_str() {
        "m.id.user" => {
            // If the field user is not empty then we can parse it into MIdUserIdentifier
            if let Some(user) = &identifier_type.user {
                let user_clone = user.clone();
                return IdentifierType::MIdUser(MIdUserIdentifier { user: user_clone });
            }
            IdentifierType::None
        }
        "m.id.thirdparty" => {
            // If the fields medium and address are not empty we can parse them into MIdThirdpartyIdentifier
            if let (Some(medium), Some(address)) =
                (&identifier_type.medium, &identifier_type.address)
            {
                let medium_clone = medium.clone();
                let addr_clone = address.clone();
                return IdentifierType::MIdThirdparty(MIdThirdpartyIdentifier {
                    medium: medium_clone,
                    address: addr_clone,
                });
            }
            IdentifierType::None
        }
        // If the fields phone and country are not empty we can parse them into MIdPhoneIdentifier
        "m.id.phone" => {
            if let (Some(phone), Some(country)) = (&identifier_type.phone, &identifier_type.country)
            {
                let phone_clone = phone.clone();
                let country_clone = country.clone();
                return IdentifierType::MIdPhone(MIdPhoneIdentifier {
                    phone: phone_clone,
                    country: country_clone,
                });
            }
            IdentifierType::None
        }
        _ => IdentifierType::None,
    }
}
