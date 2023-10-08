use crate::Value;

/// Data container for response of [`monitor::Endpoint::Status`](crate::monitor::Endpoint::Status)
///
/// See also [`monitor::Monitor`](crate::monitor::Monitor)
///
#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Status(pub Value);

#[cfg(feature = "ureq")]
impl TryFrom<ureq::Response> for Status {
    type Error = crate::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let status = response.status();

        if !(200..300).contains(&status) {
            return Err(crate::Error::HttpError(
                status,
                response.status_text().to_string(),
            ));
        }

        response.into_json::<Self>().map_err(Self::Error::from)
    }
}
