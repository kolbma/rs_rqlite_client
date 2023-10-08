use lazy_static::lazy_static;

use rqlite_client::{response, state, Connection, Error, Query, RequestBuilder};

struct ImplRequestTest {}

impl<T> RequestBuilder<T> for ImplRequestTest
where
    T: state::State,
{
    fn run(&self, query: &Query<T>) -> response::Result {
        Result::Err(Error::ResultError(format!(
            "ImplRequestTest is dummy impl: {query}"
        )))
    }
}

const TEST_CONNECTION_URL: &str = "http://localhost:4001/";

#[cfg(feature = "url")]
lazy_static! {
    static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL).unwrap();
}
#[cfg(not(feature = "url"))]
lazy_static! {
    static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL);
}

#[test]
fn request_test() {
    let q = TEST_CONNECTION.query();
    let r = ImplRequestTest {}.run(&q.set_sql_str("SELECT 1"));

    assert!(r.is_err());
    #[cfg(any(feature = "url", feature = "percent_encoding"))]
    {
        assert_eq!(
        &format!("{:?}", r.unwrap_err()),
        "ResultError(\"ImplRequestTest is dummy impl: http://localhost:4001/db/query?q=SELECT%201\")"
    );
    }
    #[cfg(not(any(feature = "url", feature = "percent_encoding")))]
    {
        assert_eq!(
            &format!("{:?}", r.unwrap_err()),
            "ResultError(\"ImplRequestTest is dummy impl: http://localhost:4001/db/query?q=SELECT 1\")"
        );
    }
}
